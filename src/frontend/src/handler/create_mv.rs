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

use pgwire::pg_response::{PgResponse, StatementType};
use risingwave_common::error::Result;
use risingwave_pb::catalog::Table as ProstTable;
use risingwave_sqlparser::ast::{ObjectName, Query};

use crate::binder::Binder;
use crate::optimizer::property::Distribution;
use crate::optimizer::PlanRef;
use crate::planner::Planner;
use crate::session::{OptimizerContext, OptimizerContextRef, SessionImpl};

/// Generate create MV plan, return plan and mv table info.
pub fn gen_create_mv_plan(
    session: &SessionImpl,
    context: OptimizerContextRef,
    query: Box<Query>,
    name: ObjectName,
) -> Result<(PlanRef, ProstTable)> {
    let (schema_name, table_name) = Binder::resolve_table_name(name)?;
    let (database_id, schema_id) = session
        .env()
        .catalog_reader()
        .read_guard()
        .check_relation_name_duplicated(session.database(), &schema_name, &table_name)?;

    let bound = {
        let mut binder = Binder::new(
            session.env().catalog_reader().read_guard(),
            session.database().to_string(),
        );
        binder.bind_query(*query)?
    };

    let mut plan_root = Planner::new(context).plan_query(bound)?;
    plan_root.set_required_dist(Distribution::any().clone());
    let materialize = plan_root.gen_create_mv_plan(table_name)?;
    let table = materialize.table().to_prost(schema_id, database_id);
    let plan: PlanRef = materialize.into();

    Ok((plan, table))
}

pub async fn handle_create_mv(
    context: OptimizerContext,
    name: ObjectName,
    query: Box<Query>,
) -> Result<PgResponse> {
    let session = context.session_ctx.clone();

    let (table, stream_plan) = {
        let (plan, table) = gen_create_mv_plan(&session, context.into(), query, name)?;
        let stream_plan = plan.to_stream_prost();
        (table, stream_plan)
    };

    let catalog_writer = session.env().catalog_writer();
    catalog_writer
        .create_materialized_view(table, stream_plan)
        .await?;

    Ok(PgResponse::empty_result(
        StatementType::CREATE_MATERIALIZED_VIEW,
    ))
}
