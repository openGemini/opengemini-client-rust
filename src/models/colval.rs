// Copyright 2024 openGemini Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use zigzag::ZigZag;

#[derive(Debug, Serialize, Deserialize)]
pub struct ColVal {
    val: Vec<u8>,
    offset: Vec<i32>,
    bitmap: Vec<u8>,
    bit_map_offset: i32,
    len: i32,
    nil_count: i32,
}

impl ColVal {
    pub fn marshal(&self) -> io::Result<Vec<u8>> {
        let mut baos = Vec::new();

        // Write encoded values
        baos.write_all(&ZigZag::encode(self.len as i64).to_le_bytes())?;
        baos.write_all(&ZigZag::encode(self.nil_count as i64).to_le_bytes())?;
        baos.write_all(&ZigZag::encode(self.bit_map_offset as i64).to_le_bytes())?;

        // Write val
        baos.write_all(&(self.val.len() as i32).to_le_bytes())?;
        if !self.val.is_empty() {
            baos.write_all(&self.val)?;
        }

        // Write bitmap
        baos.write_all(&(self.bitmap.len() as i32).to_le_bytes())?;
        if !self.bitmap.is_empty() {
            baos.write_all(&self.bitmap)?;
        }

        // Write offset
        baos.write_all(&(self.offset.len() as i32).to_le_bytes())?;
        for &off in &self.offset {
            baos.write_all(&off.to_le_bytes())?;
        }

        Ok(baos)
    }
}
