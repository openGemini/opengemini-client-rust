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
pub struct Field {
    field_type: i32,
    name: String,
}

impl Field {
    pub fn marshal(&self) -> io::Result<Vec<u8>> {
        let mut baos = Vec::new();

        // Write name length and name bytes
        let name_bytes = self.name.as_bytes();
        baos.write_all(&(name_bytes.len() as u16).to_le_bytes())?;
        baos.write_all(name_bytes)?;

        // Write encoded type
        baos.write_all(&ZigZag::encode(self.field_type as i64).to_le_bytes())?;

        Ok(baos)
    }
}
