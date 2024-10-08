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

use std::time::Duration;

pub struct Address {
    host: String,
    port: u8,
}

pub struct BatchConfig {
    batchInterval: Duration,
    batchSize: u8,
}

type AuthType = u8;

pub struct AuthConfig {
    authType: AuthType,
    username: String,
    password: String,
    token: String,
}


pub struct Config {
    Address: Vec<Address>,
    batch_config: BatchConfig,
    timeout: Duration,
    connectionTimeout: Duration,
    gzip_enabled: bool,
    auth_config: AuthConfig,
}
