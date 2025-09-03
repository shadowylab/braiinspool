//! Braiins Pool client builder

#[cfg(feature = "socks")]
use std::net::SocketAddr;
use std::time::Duration;

#[cfg(feature = "socks")]
use reqwest::Proxy;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder};

use crate::client::BraiinsPoolClient;
use crate::error::Error;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);

/// Braiins Pool client builder
#[derive(Debug, Clone)]
pub struct BraiinsPoolClientBuilder {
    /// API key
    pub api_key: String,
    /// Timeout
    pub timeout: Duration,
    /// Socks5 proxy
    #[cfg(feature = "socks")]
    pub proxy: Option<SocketAddr>,
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
            #[cfg(feature = "socks")]
            proxy: None,
        }
    }

    /// Set timeout (default: 60 sec)
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set proxy
    #[inline]
    #[cfg(feature = "socks")]
    pub fn proxy(mut self, proxy: SocketAddr) -> Self {
        self.proxy = Some(proxy);
        self
    }

    /// Build client
    pub fn build(self) -> Result<BraiinsPoolClient, Error> {
        let mut auth_value = HeaderValue::from_str(&self.api_key)?;
        auth_value.set_sensitive(true);

        let mut headers: HeaderMap = HeaderMap::with_capacity(1);
        headers.insert("Pool-Auth-Token", auth_value);

        let mut builder: ClientBuilder = Client::builder().default_headers(headers);

        // Set timeout
        builder = builder.timeout(self.timeout);

        // Set proxy
        #[cfg(all(feature = "socks", not(target_arch = "wasm32")))]
        if let Some(proxy) = self.proxy {
            let proxy: String = format!("socks5h://{proxy}");
            builder = builder.proxy(Proxy::all(proxy)?);
        }

        // Build client
        let client: Client = builder.build()?;

        // Construct client
        Ok(BraiinsPoolClient::from_client(client))
    }
}
