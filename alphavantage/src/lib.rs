
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![warn(missing_docs)]

extern crate failure;
extern crate failure_derive;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate chrono;
extern crate chrono_tz;

mod client;
mod deserialize;

pub mod exchange_rate;
pub mod time_series;
pub use crate::client::Client;
pub use crate::client::Error;
