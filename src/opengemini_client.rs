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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::config::{AuthConfig, BatchConfig};

    use super::*;

    fn create_test_config() -> Config {
        Config {
            address: vec![Address {
                host: "127.0.0.1".to_string(),
                port: 8086,
            }],
            batch_config: BatchConfig {
                batch_interval: Duration::from_secs(30),
                batch_size: 100,
            },
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            gzip_enabled: true,
            auth_config: AuthConfig {
                username: "user".to_string(),
                password: "password".to_string(),
                token: None,
                auth_type: 1,
            },
        }
    }

    #[test]
    fn test_get_server_url() {
        let addresses = vec![
            Address {
                host: "127.0.0.1".to_string(),
                port: 8086,
            },
            Address {
                host: "127.0.0.2".to_string(),
                port: 8087,
            },
        ];
        let mut config = create_test_config();

        config.address = addresses;

        let client = Client::new(&config);

        let url1 = client.get_server_url();
        let url2 = client.get_server_url();

        assert!(url1.is_some());
        assert!(url2.is_some());
        assert_ne!(url1, url2);
    }

    /// Tests the `ping` method of the `Client` struct.
    ///
    /// This test sets up a `Client` with a single address and checks if the `ping` method
    /// returns `Ok(true)` when the server is reachable.
    ///
    /// Before running this test, make sure to start the server using the following Docker command:
    /// ```sh
    /// docker run -p 8086:8086 --name opengemini --rm opengeminidb/opengemini-server
    /// ```
    #[test]
    fn test_ping_success() {
        let config = create_test_config();
        let client = Client::new(&config);

        let result = client.ping(0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }
}
