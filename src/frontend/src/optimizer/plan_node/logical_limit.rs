// Copyright 2022 Singularity Data
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;

use fixedbitset::FixedBitSet;

use super::{
    BatchLimit, ColPrunable, PlanBase, PlanNode, PlanRef, PlanTreeNodeUnary, ToBatch, ToStream,
};
use crate::utils::ColIndexMapping;

/// `LogicalLimit` fetches up to `limit` rows from `offset`
#[derive(Debug, Clone)]
pub struct LogicalLimit {
    pub base: PlanBase,
    input: PlanRef,
    pub(super) limit: usize,
    pub(super) offset: usize,
}

impl LogicalLimit {
    fn new(input: PlanRef, limit: usize, offset: usize) -> Self {
        let ctx = input.ctx();
        let schema = input.schema().clone();
        let pk_indices = input.pk_indices().to_vec();
        let base = PlanBase::new_logical(ctx, schema, pk_indices);
        LogicalLimit {
            base,
            input,
            limit,
            offset,
        }
    }

    /// the function will check if the cond is bool expression
    pub fn create(input: PlanRef, limit: usize, offset: usize) -> PlanRef {
        Self::new(input, limit, offset).into()
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

impl PlanTreeNodeUnary for LogicalLimit {
    fn input(&self) -> PlanRef {
        self.input.clone()
    }
    fn clone_with_input(&self, input: PlanRef) -> Self {
        Self::new(input, self.limit, self.offset)
    }
    #[must_use]
    fn rewrite_with_input(
        &self,
        input: PlanRef,
        input_col_change: ColIndexMapping,
    ) -> (Self, ColIndexMapping) {
        (Self::new(input, self.limit, self.offset), input_col_change)
    }
}
impl_plan_tree_node_for_unary! {LogicalLimit}
impl fmt::Display for LogicalLimit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "LogicalLimit {{ limit: {}, offset: {} }}",
            self.limit, self.offset
        )
    }
}

impl ColPrunable for LogicalLimit {
    fn prune_col(&self, required_cols: &FixedBitSet) -> PlanRef {
        self.must_contain_columns(required_cols);

        let new_input = self.input.prune_col(required_cols);
        self.clone_with_input(new_input).into()
    }
}

impl ToBatch for LogicalLimit {
    fn to_batch(&self) -> PlanRef {
        let new_input = self.input().to_batch();
        let new_logical = self.clone_with_input(new_input);
        BatchLimit::new(new_logical).into()
    }
}

impl ToStream for LogicalLimit {
    fn to_stream(&self) -> PlanRef {
        panic!("there is no limit stream operator");
    }
    fn logical_rewrite_for_stream(&self) -> (PlanRef, ColIndexMapping) {
        let (input, input_col_change) = self.input.logical_rewrite_for_stream();
        let (filter, out_col_change) = self.rewrite_with_input(input, input_col_change);
        (filter.into(), out_col_change)
    }
}
