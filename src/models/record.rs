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

use crate::models::colval::ColVal;
use crate::models::feild::Field;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    col_vals: Vec<ColVal>,
    schema: Vec<Field>,
}

impl Record {
    pub fn marshal(&self) -> io::Result<Vec<u8>> {
        let mut baos = Vec::new();

        // Write Schema
        baos.write_all(&(self.schema.len() as i32).to_le_bytes())?;
        for field in &self.schema {
            let field_bytes = field.marshal()?;
            baos.write_all(&(field_bytes.len() as i32).to_le_bytes())?;
            baos.write_all(&field_bytes)?;
        }

        // Write ColVals
        baos.write_all(&(self.col_vals.len() as i32).to_le_bytes())?;
        for col_val in &self.col_vals {
            let col_val_bytes = col_val.marshal()?;
            baos.write_all(&(col_val_bytes.len() as i32).to_le_bytes())?;
            baos.write_all(&col_val_bytes)?;
        }

        Ok(baos)
    }
}
