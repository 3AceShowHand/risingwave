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
use std::hash::Hash;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use parking_lot::Mutex;

pub const EPOCH_PHYSICAL_SHIFT_BITS: u8 = 16;
pub const INVALID_EPOCH: u64 = 0;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct Epoch(u64);

impl Epoch {
    pub fn init() -> Self {
        Epoch(Epoch::physical_now() << EPOCH_PHYSICAL_SHIFT_BITS)
    }

    pub fn physical_time(&self) -> u64 {
        self.0 >> EPOCH_PHYSICAL_SHIFT_BITS
    }

    pub fn into_inner(self) -> u64 {
        self.0
    }

    pub fn next(&self) -> Epoch {
        let physical_now = Epoch::physical_now();
        if physical_now <= self.physical_time() {
            Epoch(self.0 + 1)
        } else {
            Epoch(physical_now << EPOCH_PHYSICAL_SHIFT_BITS)
        }
    }

    // TODO: use a monotonic library to replace SystemTime.
    pub fn physical_now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}

impl From<u64> for Epoch {
    fn from(e: u64) -> Self {
        Epoch(e)
    }
}

impl fmt::Display for Epoch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

pub trait EpochGenerator: Sync + Send + 'static {
    fn generate(&self) -> Epoch;
}

pub type EpochGeneratorRef = Arc<dyn EpochGenerator>;

pub struct MemEpochGenerator {
    current_epoch: Mutex<Epoch>,
}

impl Default for MemEpochGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl MemEpochGenerator {
    pub fn new() -> Self {
        MemEpochGenerator {
            current_epoch: Mutex::new(Epoch::init()),
        }
    }
}

impl EpochGenerator for MemEpochGenerator {
    fn generate(&self) -> Epoch {
        let mut ce = self.current_epoch.lock();
        *ce = ce.next();
        *ce
    }
}

#[cfg(test)]
mod tests {
    use risingwave_common::error::Result;

    use super::*;

    #[test]
    fn test_epoch_generator() -> Result<()> {
        let generator = MemEpochGenerator::new();
        let mut pre = generator.generate();
        loop {
            let epoch = generator.generate();
            assert!(epoch > pre);
            if epoch.physical_time() > pre.physical_time() {
                break;
            }
            pre = epoch;
        }
        Ok(())
    }
}
