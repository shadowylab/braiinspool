//! Braiins Pool client builder

use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder};

use crate::client::BraiinsPoolClient;
use crate::error::Error;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);

/// Braiins Pool client builder
#[derive(Debug, Clone)]
pub struct BraiinsPoolClientBuilder {
    api_key: String,
    timeout: Duration,
}

impl BraiinsPoolClientBuilder {
    /// Construct a new builder
    pub fn new<T>(api_key: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            api_key: api_key.into(),
            timeout: DEFAULT_TIMEOUT,
        }
    }

    /// Set timeout (default: 60 sec)
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Build client
    pub fn build(self) -> Result<BraiinsPoolClient, Error> {
        let mut auth_value = HeaderValue::from_str(&self.api_key)?;
        auth_value.set_sensitive(true);

        let mut headers: HeaderMap = HeaderMap::with_capacity(1);
        headers.insert("Pool-Auth-Token", auth_value);

        let builder: ClientBuilder = Client::builder()
            .default_headers(headers)
            .timeout(self.timeout);
        let client: Client = builder.build()?;

        Ok(BraiinsPoolClient::from_client(client))
    }
}
