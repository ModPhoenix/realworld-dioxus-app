use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub email: String,
    pub token: String,
    pub username: String,
    pub image: String,
    pub bio: Option<String>,
}
