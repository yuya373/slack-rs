use reqwest::async::{Client, RequestBuilder};
use reqwest::header::HeaderMap;

pub fn build_get(client: &Client, url: &str, token: &str) -> RequestBuilder {
    let mut h = HeaderMap::new();
    h.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );
    client.get(url).headers(h)
}

#[derive(Debug, Deserialize)]
pub struct ResponseMetadata {
    next_cursor: String,
}
