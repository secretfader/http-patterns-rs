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

#[macro_use] // TODO: update for Rust 2018 (once crate has support)
extern crate validator_derive;

mod error;
mod filters;
mod handlers;
mod models;

use warp::Filter;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("127.0.0.1:{}", &port).parse::<std::net::SocketAddr>()?;
    let routes = filters::users().recover(handlers::recover);

    tracing::info!("launching on address {}", &addr);
    warp::serve(routes).run(addr).await;
    Ok(())
}
