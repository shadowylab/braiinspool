// Copyright (c) 2021-2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::collections::HashMap;

use crate::util::deserialize_number_from_string;

#[derive(Deserialize, Debug)]
pub struct CheckTorConnection {
    #[serde(rename = "IsTor")]
    pub is_tor: bool,
}

#[derive(Deserialize, Debug)]
pub struct GenericResult<T> {
    pub btc: T,
}

#[derive(Deserialize, Debug)]
pub struct EmptyData {}

#[derive(Deserialize, Debug)]
pub struct PoolStats {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub luck_b10: f32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub luck_b50: f32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub luck_b250: f32,
    pub hash_rate_unit: String,
    pub pool_scoring_hash_rate: f64,
    pub pool_active_workers: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub round_probability: f32,
    pub round_started: u32,
    pub round_duration: u32,
}

#[derive(Deserialize, Debug)]
pub struct UserProfile {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub confirmed_reward: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub unconfirmed_reward: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub estimated_reward: f64,
    pub hash_rate_unit: String,
    pub hash_rate_5m: f64,
    pub hash_rate_60m: f64,
    pub hash_rate_24h: f64,
    pub hash_rate_scoring: f64,
    pub hash_rate_yesterday: f64,
    pub low_workers: u32,
    pub off_workers: u32,
    pub ok_workers: u32,
    pub dis_workers: u32,
}

#[derive(Deserialize, Debug)]
pub struct DailyReward {
    pub date: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total_reward: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub mining_reward: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub bos_plus_reward: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub referral_bonus: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub referral_reward: f64,
}

#[derive(Deserialize, Debug)]
pub struct DailyRewards {
    pub daily_rewards: Vec<DailyReward>,
}

#[derive(Deserialize, Debug)]
pub struct Worker {
    pub state: String,
    pub last_share: u64,
    pub hash_rate_unit: String,
    pub hash_rate_scoring: f64,
    pub hash_rate_5m: f64,
    pub hash_rate_60m: f64,
    pub hash_rate_24h: f64,
}

#[derive(Deserialize, Debug)]
pub struct Workers {
    pub workers: HashMap<String, Worker>,
}
