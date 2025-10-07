//! Models

use std::collections::HashMap;

use serde::Deserialize;

use crate::util::deserialize_number_from_string;

#[derive(Deserialize)]
pub(crate) struct BtcResponse<T> {
    pub btc: T,
}

/// Hash rate unit
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub enum HashRateUnit {
    /// Hash per second (1 hash/sec)
    #[serde(rename = "H/s", alias = "h/s")]
    H,
    /// Kilohash per second (1,000 hashes/sec)
    #[serde(rename = "Kh/s", alias = "KH/s", alias = "kh/s")]
    KH,
    /// Megahash per second (1,000,000 hashes/sec)
    #[serde(rename = "Mh/s", alias = "MH/s", alias = "mh/s")]
    MH,
    /// Gigahash per second (1,000,000,000 hashes/sec)
    #[serde(rename = "Gh/s", alias = "GH/s", alias = "gh/s")]
    GH,
    /// Terahash per second (1,000,000,000,000 hashes/sec)
    #[serde(rename = "Th/s", alias = "TH/s", alias = "th/s")]
    TH,
    /// Petahash per second (1,000,000,000,000,000 hashes/sec)
    #[serde(rename = "Ph/s", alias = "PH/s", alias = "ph/s")]
    PH,
    /// Exahash per second (1,000,000,000,000,000,000 hashes/sec)
    #[serde(rename = "Eh/s", alias = "EH/s", alias = "eh/s")]
    EH,
    /// Zettahash per second (1,000,000,000,000,000,000,000 hashes/sec)
    #[serde(rename = "Zh/s", alias = "ZH/s", alias = "zh/s")]
    ZH,
    /// Yottahash per second (1,000,000,000,000,000,000,000,000 hashes/sec)
    #[serde(rename = "Yh/s", alias = "YH/s", alias = "yh/s")]
    YH,
}

/// Block
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Block {
    /// Unix time when given block was found
    pub date_found: u64,
    /// Duration of the round leading to given block
    pub mining_duration: u32,
    /// Number of shares collected during the round
    pub total_shares: u64,
    /// State of given block
    pub state: String,
    /// Number of confirmations left
    pub confirmations_left: u32,
    /// Block value
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub value: f64,
    /// User reward for the given block
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub user_reward: f64,
    /// Pool scoring hash rate at the time when block was found
    pub pool_scoring_hash_rate: f64,
}

/// Pool stats
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PoolStats {
    /// Unit used for the hash rate values
    pub hash_rate_unit: HashRateUnit,
    /// Pool hash rate for the last 5 minutes
    pub pool_5m_hash_rate: f64,
    /// Pool hash rate for the last 60 minutes
    pub pool_60m_hash_rate: f64,
    /// Pool hash rate for the last 24 hours
    pub pool_24h_hash_rate: f64,
    /// Update timestamp
    pub update_ts: u64,
    /// Blocks
    pub blocks: HashMap<String, Block>,
    /// FPPS rate
    pub fpps_rate: f64,
}

/// User profile
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct UserProfile {
    /// Cumulative all-time reward
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub all_time_reward: f64,
    /// Unit used for the hash rate values
    pub hash_rate_unit: HashRateUnit,
    /// Average hash rate for the last 5 minutes
    pub hash_rate_5m: f64,
    /// Average hash rate for the last 60 minutes
    pub hash_rate_60m: f64,
    /// Average hash rate for the last 24 hours
    pub hash_rate_24h: f64,
    /// Average hash rate for the previous UTC day
    pub hash_rate_yesterday: f64,
    /// Number of workers with `low` state
    pub low_workers: u32,
    /// Number of workers with `off` state
    pub off_workers: u32,
    /// Number of workers with `ok` state
    pub ok_workers: u32,
    /// Number of workers with disabled monitoring
    pub dis_workers: u32,
    /// Current reward balance
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub current_balance: f64,
    /// Confirmed reward for this day
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub today_reward: f64,
    /// Estimated reward for the current block
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub estimated_reward: f64,
    /// Active shares for last 5 minutes
    pub shares_5m: u32,
    /// Active shares for last 60 minutes
    pub shares_60m: u32,
    /// Active shares for last 24 hours
    pub shares_24h: u32,
    /// Active shares for yesterday
    pub shares_yesterday: u32,
}

/// Daily reward
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct DailyReward {
    /// Unix time (the first second of the date)
    pub date: u64,
    /// The sum of all reward types for the day
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total_reward: f64,
    /// The standard mining reward
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub mining_reward: f64,
    /// The amount refunded (pool fee refund) for mining with Braiins OS
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub bos_plus_reward: f64,
    /// Bonus received by being referred to Braiins OS
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub referral_bonus: f64,
    /// Reward earned for HR referred to Braiins OS
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub referral_reward: f64,
    /// Calculation date timestamp
    pub calculation_date: u64,
}

/// Daily rewards
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct DailyRewards {
    /// Daily rewards
    pub daily_rewards: Vec<DailyReward>,
}

/// Worker
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Worker {
    /// State of the worker (`ok`/`low`/`off`/`dis`)
    pub state: String,
    /// Unix time of the last accepted share
    pub last_share: u64,
    /// Unit used for the hash rate values
    pub hash_rate_unit: HashRateUnit,
    /// Current scoring hash rate
    pub hash_rate_scoring: f64,
    /// Average hash rate for the last 5 minutes
    pub hash_rate_5m: f64,
    /// Average hash rate for the last 60 minutes
    pub hash_rate_60m: f64,
    /// Average hash rate for the last 24 hours
    pub hash_rate_24h: f64,
    /// Active shares for last 5 minutes
    pub shares_5m: u64,
    /// Active shares for last 60 minutes
    pub shares_60m: u64,
    /// Active shares for last 24 hours
    pub shares_24h: u64,
}

/// Workers
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Workers {
    /// Workers
    pub workers: HashMap<String, Worker>,
}
