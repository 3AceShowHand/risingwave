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

use prost::Message;
use risingwave_pb::hummock::{CompactTaskAssignment, CompactTaskRefId};

use crate::model::MetadataModel;

/// `cf(compact_task_assignment)`: `CompactTaskRefId` -> `CompactTaskAssignment`
const HUMMOCK_COMPACT_TASK_ASSIGNMENT: &str = "cf/compact_task_assignment";

/// `AssignedCompactTasks` tracks compact tasks assigned to context id.
impl MetadataModel for CompactTaskAssignment {
    type ProstType = CompactTaskAssignment;
    type KeyType = CompactTaskRefId;

    fn cf_name() -> String {
        HUMMOCK_COMPACT_TASK_ASSIGNMENT.to_string()
    }

    fn to_protobuf(&self) -> Self::ProstType {
        self.clone()
    }

    fn to_protobuf_encoded_vec(&self) -> Vec<u8> {
        self.encode_to_vec()
    }

    fn from_protobuf(prost: Self::ProstType) -> Self {
        prost
    }

    fn key(&self) -> risingwave_common::error::Result<Self::KeyType> {
        Ok(CompactTaskRefId {
            id: self.compact_task.as_ref().unwrap().task_id,
        })
    }
}
