use crate::{
    error::ServiceError,
    models::{NewUserRequest, User},
};
use validator::Validate;

/// Load the first two records from the JSON API, deserialize as
/// the `User` model, and return to the client.
pub async fn list() -> Result<impl warp::Reply, warp::Rejection> {
    let req: Vec<User> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/users")
        .send()
        .await
        .map_err(|e| warp::reject::custom(ServiceError::External(e)))?
        .json::<Vec<User>>()
        .await
        .map_err(|e| warp::reject::custom(ServiceError::InvalidPayload(e.to_string())))?
        .into_iter()
        .take(2)
        .collect();

    Ok(warp::reply::json(&req))
}

/// Validate payload and attempt to pass the validated data along
/// to the JSON API.
pub async fn create(req: NewUserRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let req = req
        .validate()
        .map_err(|e| warp::reject::custom(ServiceError::Validation(e)))?;

    reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/users")
        .json(&req)
        .send()
        .await
        .map_err(|e| warp::reject::custom(ServiceError::External(e)))?;

    Ok(warp::http::StatusCode::CREATED)
}
