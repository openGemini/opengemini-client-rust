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
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

pub trait ToValue {
    type Error;

    fn to_value() -> Result<Value, Self::Error>;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, PartialOrd)]
pub struct Point {
    pub measurement: String,
    pub tags: BTreeMap<String, String>,
    pub fields: BTreeMap<String, Value>,
    pub timestamp: u64,
}

impl Point {
    pub fn add_tag<K: ToString, V: ToString>(&mut self, key: K, value: V) -> &mut Self {
        let key = key.to_string();
        let value = value.to_string();
        self.tags.insert(key, value);
        self
    }

    pub fn add_field<K: ToString>(&mut self, key: K, value: Value) -> &mut Self {
        let key = key.to_string();
        self.fields.insert(key, value);
        self
    }
}
