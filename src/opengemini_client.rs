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

use std::sync::atomic::{AtomicBool, AtomicI32};

use crate::config::{Address, Config};
use crate::error::ClientError;
use reqwest::Client as HttpClient;

pub struct Endpoint {
    pub url: String,
    pub is_down: AtomicBool,
}

pub struct Client {
    config: Config,
    endpoints: Vec<Endpoint>,
    cli: HttpClient,
    prev_idx: AtomicI32,
}

impl Client {
    pub fn new(cfg: &Config) -> Self {
        let client = HttpClient::new();
        let endpoints = build_endpoints(cfg.address.clone());
        Client {
            config: cfg.clone(),
            endpoints,
            cli: client,
            prev_idx: AtomicI32::new(0),
        }
    }

    pub fn ping(&self) -> Result<(), ClientError> {
        todo!();
    }

    pub fn query(&self) -> Result<(), ClientError> {
        todo!();
    }

    pub fn close(&self) -> Result<(), ClientError> {
        todo!();
    }
}

pub fn build_endpoints(addresses: Vec<Address>) -> Vec<Endpoint> {
    addresses
        .into_iter()
        .map(|addr| {
            let url = format!("http://{}:{}", addr.host, addr.port);
            Endpoint {
                url,
                is_down: AtomicBool::new(false),
            }
        })
        .collect()
}
