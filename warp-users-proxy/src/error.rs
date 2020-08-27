// Copyright 2020 Nicholas Young.
//
// Use of this source code is governed by the Mozilla Public License
// ("MPL"), version 2.0, which can be found in the included LICENSE
// file or at https://www.mozilla.org/en-US/MPL/2.0.

use std::convert;
use warp::reject;

/// Error variants that may be created during program operation
#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Validation error {0}")]
    Validation(validator::ValidationErrors),
    #[error("Backend service unavailable")]
    External(reqwest::Error),
    #[error("Invalid payload from backend {0}")]
    InvalidPayload(String),
}

impl reject::Reject for ServiceError {}

impl convert::From<ServiceError> for reject::Rejection {
    fn from(e: ServiceError) -> Self {
        reject::custom(e)
    }
}

impl convert::From<validator::ValidationErrors> for ServiceError {
    fn from(e: validator::ValidationErrors) -> Self {
        ServiceError::Validation(e)
    }
}

impl convert::From<reqwest::Error> for ServiceError {
    fn from(e: reqwest::Error) -> Self {
        ServiceError::External(e)
    }
}
