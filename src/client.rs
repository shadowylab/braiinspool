//! Client

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder, Response};
use serde::de::DeserializeOwned;
use url::Url;

use crate::error::Error;
use crate::model::{BtcResponse, DailyRewards, PoolStats, UserProfile, Workers};

const BASE_URL: &str = "https://pool.braiins.com";

/// Braiins Pool client
#[derive(Debug, Clone)]
pub struct BraiinsPoolClient {
    url: Url,
    client: Client,
}

impl BraiinsPoolClient {
    /// Construct a new Braiins Pool client
    pub fn new(api_key: &str) -> Result<Self, Error> {
        // Parse base URL
        let url: Url = Url::parse(BASE_URL)?;

        let mut headers = HeaderMap::new();
        let mut auth_value = HeaderValue::from_str(api_key)?;
        auth_value.set_sensitive(true);
        headers.insert("Pool-Auth-Token", auth_value);

        let client: ClientBuilder = Client::builder().default_headers(headers);

        Ok(Self {
            url,
            client: client.build()?,
        })
    }

    async fn request<T>(&self, url: Url) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let res: Response = self.client.get(url).send().await?;
        let res: BtcResponse<T> = res.json().await?;
        Ok(res.btc)
    }

    /// Get pool stats
    pub async fn pool_stats(&self) -> Result<PoolStats, Error> {
        let url: Url = self.url.join("/stats/json/btc")?;
        self.request(url).await
    }

    /// Get user profile
    pub async fn user_profile(&self) -> Result<UserProfile, Error> {
        let url: Url = self.url.join("/accounts/profile/json/btc")?;
        self.request(url).await
    }

    /// Get daily rewards
    pub async fn daily_rewards(&self) -> Result<DailyRewards, Error> {
        let url: Url = self.url.join("/accounts/rewards/json/btc")?;
        self.request(url).await
    }

    /// Get workers
    pub async fn workers(&self) -> Result<Workers, Error> {
        let url: Url = self.url.join("/accounts/workers/json/btc")?;
        self.request(url).await
    }
}
