//! Error

use std::fmt;

use reqwest::header::InvalidHeaderValue;

/// Braiins Pool API Error
#[derive(Debug)]
pub enum Error {
    /// Url parse error
    Url(url::ParseError),
    /// Reqwest error
    Reqwest(reqwest::Error),
    /// Invalid header value
    InvalidHeaderValue(InvalidHeaderValue),
    /// invalid API key
    InvalidApiKey,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Url(e) => e.fmt(f),
            Self::Reqwest(e) => e.fmt(f),
            Self::InvalidHeaderValue(e) => e.fmt(f),
            Self::InvalidApiKey => f.write_str("Invalid API Key"),
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Self::Url(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(e: InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue(e)
    }
}
