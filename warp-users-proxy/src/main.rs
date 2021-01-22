// Copyright 2020 Nicholas Young.
//
// Use of this source code is governed by the Mozilla Public License
// ("MPL"), version 2.0, which can be found in the included LICENSE
// file or at https://www.mozilla.org/en-US/MPL/2.0.

#![deny(missing_docs, missing_debug_implementations)]

//! `warp-users-proxy` implements typical REST API patterns, including
//! versioning, input payload validation, working with external APIs,
//! providing JSON-encoded responses to clients (including custom
//! error response objects). This implementation also contains
//! various integration tests.
//!
//! Following common Warp development patterns, this implementation
//! is structured as follows:
//!
//! Models contain data that is required to be deserialized from a
//! network request or serialized by this appliction for delivery
//! over HTTP.
//!
//! Filters contain routing tables and preconditions that must be
//! fulfilled by each request in order to be routed to a given path.
//!
//! Handlers return properly formatted responses to the client after
//! filters have been executed.

mod error;
mod filters;
mod handlers;
mod models;

use hyper::{server::Server, service::make_service_fn};
use listenfd::ListenFd;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("127.0.0.1:{}", &port).parse::<std::net::SocketAddr>()?;
    let routes = filters::users().recover(handlers::recover);

    let svc = warp::service(routes);
    let svc_builder = make_service_fn(|_: _| {
        let svc = svc.clone();
        async move { Ok::<_, std::convert::Infallible>(svc) }
    });

    let server = if let Some(fd) = ListenFd::from_env().take_tcp_listener(0)? {
        Server::from_tcp(fd)?
    } else {
        Server::bind(&addr)
    };

    Ok(server.serve(svc_builder).await?)
}
