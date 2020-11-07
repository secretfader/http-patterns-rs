// Copyright 2020 Nicholas Young.
//
// Use of this source code is governed by the Mozilla Public License
// ("MPL"), version 2.0, which can be found in the included LICENSE
// file or at https://www.mozilla.org/en-US/MPL/2.0.

use serde::{Deserialize, Serialize};
use validator::Validate;

/// A User record, as retrieved from the data source
#[derive(Deserialize, Serialize)]
pub struct User {
    id: usize,
    email: String,
    name: String,
}

/// Payload required to create a new User record
#[derive(Deserialize, Serialize, Validate)]
pub struct NewUserRequest {
    /// Email address
    #[validate(email(message = "Please supply a valid email address"))]
    pub email: String,
    /// Name of the user who is registering
    #[validate(length(min = 3, message = "Name must be at least three characters long"))]
    pub name: String,
}
