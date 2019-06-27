use std::sync::Arc;
use std::thread;
use std::time::Duration;

extern crate gotts_oracle_alphavantage;
use gotts_oracle_alphavantage as alphavantage;

extern crate gotts_oracle_api;
use gotts_oracle_api as api;

fn main() {
    //create alphavantage client
    let shared_client = Arc::new(alphavantage::Client::new("2BY6TAJHCM9Z7HQT"));

    //start api server
    api::start_rest_apis(shared_client.clone(), "127.0.0.1:8008".to_string(), None);

    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}
