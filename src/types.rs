use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct GenericError {
    pub errors: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub email: String,
    pub token: String,
    pub username: String,
    pub image: String,
    pub bio: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SignUpFormData {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SignUpFormDataRequest {
    pub user: SignUpFormData,
}
