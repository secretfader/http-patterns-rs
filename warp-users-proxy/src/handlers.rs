// Copyright 2020 Nicholas Young.
//
// Use of this source code is governed by the Mozilla Public License
// ("MPL"), version 2.0, which can be found in the included LICENSE
// file or at https://www.mozilla.org/en-US/MPL/2.0.

pub mod users;

use crate::error::ServiceError;
use warp::{http::StatusCode, Rejection, Reply};

/// If a request is rejected further up the filter chain or in a
/// handler, this function renders an appropriate client-facing error
/// response.
pub async fn recover(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let mut code: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
    let mut message: String = "".to_string();
    let mut context: Option<ValidationContext> = None;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND".to_string();
    } else if let Some(ServiceError::Validation(e)) = err.find() {
        code = StatusCode::UNPROCESSABLE_ENTITY;
        message = "Validation failed".to_string();
        context = Some(e.errors().to_owned());
    } else if let Some(ServiceError::External(e)) = err.find() {
        code = StatusCode::SERVICE_UNAVAILABLE;
        message = e.to_string();
    } else if let Some(ServiceError::InvalidPayload(e)) = err.find() {
        code = StatusCode::BAD_GATEWAY;
        message = e.to_string();
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = e.to_string();
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "HTTP method not allowed".to_string();
    } 

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message,
        context,
    });

    Ok(warp::reply::with_status(json, code))
}

type ValidationContext = std::collections::HashMap<&'static str, validator::ValidationErrorsKind>;

// Error format that is returned to clients, usually formatted as JSON
#[derive(serde::Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
    context: Option<ValidationContext>,
}
