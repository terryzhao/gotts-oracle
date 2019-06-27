use crate::rest::*;
use crate::router::{Handler, ResponseFuture};
use crate::web::*;
use futures::{stream, Stream};
use hyper::{Body, Request, StatusCode};
use std::sync::Weak;
use std::thread;

extern crate gotts_oracle_alphavantage;
use alphavantage::exchange_rate::ExchangeRate;
use gotts_oracle_alphavantage as alphavantage;

pub struct IndexHandler {
    pub list: Vec<String>,
}

impl IndexHandler {}

impl Handler for IndexHandler {
    fn get(&self, _req: Request<Body>) -> ResponseFuture {
        json_response_pretty(&self.list)
    }
}

pub struct ExchangeHandler {
    pub client: Weak<alphavantage::Client>,
}

impl ExchangeHandler {
    fn get_rate(&self, req: Request<Body>) -> Result<f64, Error> {
        let query = must_get_query!(req);
        let params = QueryParams::from(query);
        let from = parse_param_no_err!(params, "from", "USD".to_owned());
        let to = parse_param_no_err!(params, "to", "CNY".to_owned());
        let arc_client = w(&self.client)?;

        let exchange_rate = crossbeam::scope(|scope| {
            let handle = scope.spawn(move |_| -> Result<ExchangeRate, Error> {
                let exchange_result = arc_client.get_exchange_rate(&from, &to);
                let result = match exchange_result {
                    Ok(result) => Ok(result),
                    Err(_e) => Err(ErrorKind::RequestError(
                        "query alphavantage failed!".to_owned(),
                    ))?,
                };

                result
            });

            handle.join().unwrap()
        });

        let result = exchange_rate.unwrap();

        let rate = match result {
            Ok(result) => Ok(result.rate),
            Err(_e) => Ok(0.00),
        };

        rate
    }
}

impl Handler for ExchangeHandler {
    fn get(&self, req: Request<Body>) -> ResponseFuture {
        result_to_response(self.get_rate(req))
    }
}
