#![warn(clippy::all)]
use std::env;
use warp::http::Method;
use warp::Filter;

use handle_errors::return_error;
use tracing_subscriber::fmt::format::FmtSpan;

mod config;
mod profanity;
mod routes;
mod store;
mod types;
