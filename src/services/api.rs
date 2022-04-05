use reqwest::{header, Client};

use crate::{settings::JWT_KEY, utils::local_storage};

#[derive(Clone)]
pub struct API {
    pub client: Client,
}

impl API {
    pub fn new() -> Self {
        let jwt = local_storage::get_item(JWT_KEY);

        log::info!("jwt {:?}", jwt);

        let mut headers = header::HeaderMap::new();

        if let Some(token) = jwt {
            headers.insert(
                "Authorization",
                header::HeaderValue::from_str(&token).unwrap(),
            );
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { client }
    }

    pub fn create_url(path: &str) -> String {
        "https://api.realworld.io/api".to_string() + path
    }
}
