mod server_api;

use self::server_api::ExchangeHandler;
use self::server_api::IndexHandler;

use crate::auth::{BasicAuthMiddleware, GRIN_BASIC_REALM};
use crate::rest::*;
use crate::router::{Router, RouterError};
use std::net::SocketAddr;
use std::sync::Arc;

extern crate gotts_oracle_alphavantage;
use gotts_oracle_alphavantage as alphavantage;

/// Start all server HTTP handlers. Register all of them with Router
/// and runs the corresponding HTTP server.
///
/// Hyper currently has a bug that prevents clean shutdown. In order
/// to avoid having references kept forever by handlers, we only pass
/// weak references. Note that this likely means a crash if the handlers are
/// used after a server shutdown (which should normally never happen,
/// except during tests).
pub fn start_rest_apis(
    client: Arc<alphavantage::Client>,
    addr: String,
    tls_config: Option<TLSConfig>,
) -> bool {
    let mut apis = ApiServer::new();
    let mut router = build_router(client).expect("unable to build API router");

    info!("Starting HTTP API server at {}.", addr);
    let socket_addr: SocketAddr = addr.parse().expect("unable to parse socket address");
    let res = apis.start(socket_addr, router, tls_config);
    match res {
        Ok(_) => true,
        Err(e) => {
            error!("HTTP API server failed to start. Err: {}", e);
            false
        }
    }
}

pub fn build_router(client: Arc<alphavantage::Client>) -> Result<Router, RouterError> {
    let route_list = vec!["exchange".to_string()];

    let index_handler = IndexHandler { list: route_list };
    let exchange_handler = ExchangeHandler {
        client: Arc::downgrade(&client),
    };

    let mut router = Router::new();

    router.add_route("/", Arc::new(index_handler))?;
    router.add_route("/exchange", Arc::new(exchange_handler))?;

    Ok(router)
}
