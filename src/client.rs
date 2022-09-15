// Copyright (c) 2021-2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};
use reqwest::Client as ReqwestClient;
use reqwest::Proxy;
use serde::de::DeserializeOwned;

use crate::model::{
    CheckTorConnection, DailyReward, DailyRewards, GenericResult, PoolStats, UserProfile, Worker,
    Workers,
};

pub const BASE_URL: &str = "https://pool.braiins.com";

#[derive(Clone)]
pub struct Client {
    client: ReqwestClient,
}

#[derive(Debug)]
pub enum Error {
    FailedToDeserialize(String),
    ReqwestError(reqwest::Error),
    InvalidHeaderValue(InvalidHeaderValue),
    EmptyResponse,
    BadResult,
    Unauthorized,
    BadRequest,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    TooManyRequests,
    UnhandledClientError,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    UnhandledServerError,
    InvalidApiKey,
}

impl Client {
    pub fn new(api_key: &str, proxy: Option<&str>) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        let mut auth_value = HeaderValue::from_str(api_key)?;
        auth_value.set_sensitive(true);
        headers.insert("SlushPool-Auth-Token", auth_value);

        let mut client = ReqwestClient::builder().default_headers(headers);

        if let Some(proxy) = proxy {
            client = client.proxy(Proxy::all(proxy)?);
        }

        Ok(Self {
            client: client.build()?,
        })
    }

    pub async fn check_tor_connection(&self) -> Result<bool, Error> {
        let req = self.client.get("https://check.torproject.org/api/ip");
        let res = request::<CheckTorConnection>(req).await?;

        Ok(res.is_tor)
    }

    /// Get Pool Stats
    pub async fn pool_stats(&self) -> Result<PoolStats, Error> {
        let endpoint: String = format!("{}/stats/json/btc", BASE_URL);

        let req = self.client.get(endpoint);
        let res = request::<GenericResult<PoolStats>>(req).await?;

        Ok(res.btc)
    }

    /// Get User Profile
    pub async fn user_profile(&self) -> Result<UserProfile, Error> {
        let endpoint: String = format!("{}/accounts/profile/json/btc", BASE_URL);

        let req = self.client.get(endpoint);
        let res = request::<GenericResult<UserProfile>>(req).await?;

        Ok(res.btc)
    }

    /// Get Daily Rewards
    pub async fn daily_rewards(&self) -> Result<Vec<DailyReward>, Error> {
        let endpoint: String = format!("{}/accounts/rewards/json/btc", BASE_URL);

        let req = self.client.get(endpoint);
        let res = request::<GenericResult<DailyRewards>>(req).await?;

        Ok(res.btc.daily_rewards)
    }

    /// Get Workers
    pub async fn workers(&self) -> Result<HashMap<String, Worker>, Error> {
        let endpoint: String = format!("{}/accounts/workers/json/btc", BASE_URL);

        let req = self.client.get(endpoint);
        let res = request::<GenericResult<Workers>>(req).await?;

        Ok(res.btc.workers)
    }
}

async fn request<T>(req: reqwest::RequestBuilder) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let res = req.send().await?;

    match reqwest::StatusCode::as_u16(&res.status()) {
        0_u16..=399_u16 => {
            let res = res.text().await?;

            if res.is_empty() {
                return Err(Error::EmptyResponse);
            }

            if res.contains("Invalid Access Profile token") {
                return Err(Error::InvalidApiKey);
            }

            deserialize::<T>(res.as_str())
        }
        400 => Err(Error::BadRequest),
        401 => Err(Error::Unauthorized),
        402 => Err(Error::UnhandledClientError),
        403 => Err(Error::Forbidden),
        404 => Err(Error::NotFound),
        405 => Err(Error::MethodNotAllowed),
        406_u16..=428_u16 => Err(Error::UnhandledClientError),
        429 => Err(Error::TooManyRequests),
        430_u16..=499_u16 => Err(Error::UnhandledClientError),
        500 => Err(Error::InternalServerError),
        501 => Err(Error::NotImplemented),
        502 => Err(Error::BadGateway),
        503 => Err(Error::ServiceUnavailable),
        504 => Err(Error::GatewayTimeout),
        _ => Err(Error::UnhandledServerError),
    }
}

fn deserialize<T>(data: &str) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    match serde_json::from_str::<T>(data) {
        Ok(res) => Ok(res),
        Err(error) => Err(Error::FailedToDeserialize(error.to_string())),
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ReqwestError(err)
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(err: InvalidHeaderValue) -> Self {
        Error::InvalidHeaderValue(err)
    }
}
