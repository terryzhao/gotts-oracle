use std::sync::Arc;
use std::thread;
use std::time::Duration;

extern crate gotts_oracle_alphavantage;
use gotts_oracle_alphavantage as alphavantage;

extern crate gotts_oracle_api;
use gotts_oracle_api as api;

fn main() {
    let shared_client = Arc::new(alphavantage::Client::new("MY_SECRET_TOKEN"));

    ///start api server
    api::start_rest_apis(shared_client.clone(), "127.0.0.1:8008".to_string(), None);

    loop {
        thread::sleep(Duration::from_millis(1000));
    }

    std::process::exit(0);

    //    let client = alphavantage::Client::new("MY_SECRET_TOKEN");
    ////    let time_series = client.get_time_series_daily("BIDU").unwrap();
    ////    let entry = time_series.entries.last().unwrap();
    ////    println!("{:?}", entry);
    //
    //    let exchange_rate = client.get_exchange_rate("USD", "CNY").unwrap();
    //    println!("{:?}", exchange_rate);
}
