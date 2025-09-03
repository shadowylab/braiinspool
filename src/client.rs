//! Client

use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use url::Url;

use crate::builder::BraiinsPoolClientBuilder;
use crate::error::Error;
use crate::model::{BtcResponse, DailyRewards, PoolStats, UserProfile, Workers};

const BASE_URL: &str = "https://pool.braiins.com";

/// Braiins Pool client
#[derive(Debug, Clone)]
pub struct BraiinsPoolClient {
    pub(crate) url: Url,
    pub(crate) client: Client,
}

impl BraiinsPoolClient {
    /// Construct a new Braiins Pool client
    pub fn new<T>(api_key: T) -> Result<Self, Error>
    where
        T: Into<String>,
    {
        Self::builder(api_key).build()
    }

    /// Construct a new Braiins Pool client builder
    #[inline]
    pub fn builder<T>(api_key: T) -> BraiinsPoolClientBuilder
    where
        T: Into<String>,
    {
        BraiinsPoolClientBuilder::new(api_key)
    }

    /// Construct new with a custom reqwest [`Client`].
    #[inline]
    pub fn from_client(client: Client) -> Self {
        Self {
            url: Url::parse(BASE_URL).expect("Invalid base URL"),
            client,
        }
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
