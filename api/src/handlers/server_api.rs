// Copyright 2019 The Gotts Developers
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
