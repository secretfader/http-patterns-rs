// Copyright 2020 Nicholas Young.
//
// Use of this source code is governed by the Mozilla Public License
// ("MPL"), version 2.0, which can be found in the included LICENSE
// file or at https://www.mozilla.org/en-US/MPL/2.0.

use crate::handlers;
use warp::{Rejection, Reply, Filter};

/// Create two filters, each for `GET` and `POST` requests to the same
/// path: `api/v1/users`
pub fn users() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    users_list().or(users_create())
}

/// Route `GET` requests to `api/v1/users`
pub fn users_list() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "v1" / "users")
        .and(warp::get())
        .and_then(handlers::users::list)
}

/// Route `POST` requests to `api/v1/users`
pub fn users_create() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "v1" / "users")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handlers::users::create)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{handlers, models::NewUserRequest};
    use warp::{http::StatusCode, test::request};

    #[tokio::test]
    async fn invalid_create_request_empty_fields() {
        let api = users().recover(handlers::recover);
        let res = request()
            .path("/api/v1/users")
            .method("POST")
            .json(&NewUserRequest {
                email: "".to_string(),
                name: "".to_string(),
            })
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn invalid_create_request_bad_email() {
        let api = users().recover(handlers::recover);
        let res = request()
            .path("/api/v1/users")
            .method("POST")
            .json(&NewUserRequest {
                email: "z@".to_string(),
                name: "MKBHD".to_string(),
            })
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn invalid_create_request_name_too_short() {
        let api = users().recover(handlers::recover);
        let res = request()
            .path("/api/v1/users")
            .method("POST")
            .json(&NewUserRequest {
                email: "test@example.com".to_string(),
                name: "MK".to_string(),
            })
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn invalid_create_request_empty_email() {
        let api = users().recover(handlers::recover);
        let res = request()
            .path("/api/v1/users")
            .method("POST")
            .json(&NewUserRequest {
                email: "".to_string(),
                name: "MKBHD".to_string(),
            })
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn valid_create_request() {
        let api = users().recover(handlers::recover);
        let res = request()
            .path("/api/v1/users")
            .method("POST")
            .json(&NewUserRequest {
                email: "test@example.com".to_string(),
                name: "MKBHD".to_string(),
            })
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::CREATED);
    }
}
