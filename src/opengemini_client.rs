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

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use crate::config::{Address, Config};
use crate::error::ClientError;
use crate::url_const::URL_PING;
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

    pub fn ping(&self, idx: usize) -> Result<bool, ClientError> {
        if idx >= self.endpoints.len() as usize {
            return Err(ClientError::ValueError("Index out of range".to_string()));
        }
        let url: String = self.endpoints[idx].url.clone() + URL_PING;
        match reqwest::blocking::get(url) {
            Ok(response) => {
                if response.status().is_success() {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }
            Err(_) => Err(ClientError::ConnectionError),
        }
    }

    pub fn query(&self) -> Result<(), ClientError> {
        todo!();
    }

    pub fn close(&self) -> Result<(), ClientError> {
        todo!();
    }

    fn get_server_url(&self) -> Option<String> {
        let current_index = self.prev_idx.load(Ordering::SeqCst);
        let endpoint_count = self.endpoints.len() as i32;
        let url = if endpoint_count > 0 {
            let url = self.endpoints[current_index as usize % endpoint_count as usize]
                .url
                .clone();
            self.prev_idx
                .store((current_index + 1) % endpoint_count, Ordering::SeqCst);
            Some(url)
        } else {
            None
        };
        url
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
