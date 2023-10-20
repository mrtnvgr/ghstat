use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
    Result,
};

pub trait Headers {
    fn from_token(gh_token: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.append(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {gh_token}")).unwrap(),
        );
        headers
    }
}

impl Headers for HeaderMap {}

pub struct ClientBuilder;

impl ClientBuilder {
    pub fn build(headers: HeaderMap) -> Result<Client> {
        Client::builder()
            .user_agent("ghstats/0.1.0")
            .default_headers(headers)
            .build()
    }
}
