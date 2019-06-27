use failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
mod web;
pub mod auth;
pub mod client;
mod handlers;
mod rest;
mod router;

pub use crate::auth::{BasicAuthMiddleware, GRIN_BASIC_REALM};
pub use crate::handlers::start_rest_apis;
pub use crate::rest::*;
pub use crate::router::*;
pub use crate::web::*;
