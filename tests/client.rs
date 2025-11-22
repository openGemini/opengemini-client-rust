use opengemini::Client;
use opengemini::config::{Address, AuthConfig, BatchConfig, Config};
use std::time::Duration;

#[static_init::dynamic]
static CONFIG: Config = Config {
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
};

#[test]
fn test_get_server_url() {
    let mut config = CONFIG.clone();
    config.address.push(Address {
        host: "127.0.0.1".to_string(),
        port: 8087,
    });

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
    let client = Client::new(&CONFIG);

    let result = client.ping(0);
    if result.is_err() {
        log::info!("{:?}", &result);
    }
    assert!(result.is_ok());
    assert!(result.unwrap());
}
