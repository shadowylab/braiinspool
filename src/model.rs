//! Models

use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

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

impl HashRateUnit {
    fn exponent(&self) -> i32 {
        match self {
            Self::H => 1,
            Self::KH => 3,
            Self::MH => 6,
            Self::GH => 9,
            Self::TH => 12,
            Self::PH => 15,
            Self::EH => 18,
            Self::ZH => 21,
            Self::YH => 24,
        }
    }
}

/// Hashrate
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HashRate {
    unit: HashRateUnit,
    value: f64,
}

impl HashRate {
    #[inline]
    fn new(unit: HashRateUnit, value: f64) -> Self {
        Self { unit, value }
    }

    /// Get the hashrate unit.
    #[inline]
    pub fn unit(&self) -> HashRateUnit {
        self.unit
    }

    /// Get the hashrate value in [`HashRateUnit`].
    #[inline]
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Get hashrate as **hashes/sec**.
    #[inline]
    pub fn to_hashes(&self) -> f64 {
        self.value * 10f64.powi(self.unit.exponent())
    }
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
#[derive(Debug, Clone, PartialEq)]
pub struct PoolStats {
    /// Pool hash rate for the last 5 minutes
    pub pool_5m_hash_rate: HashRate,
    /// Pool hash rate for the last 60 minutes
    pub pool_60m_hash_rate: HashRate,
    /// Pool hash rate for the last 24 hours
    pub pool_24h_hash_rate: HashRate,
    /// Update timestamp
    pub update_ts: u64,
    /// Blocks
    pub blocks: HashMap<String, Block>,
    /// FPPS rate
    pub fpps_rate: f64,
}

impl<'de> Deserialize<'de> for PoolStats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            hash_rate_unit: HashRateUnit,
            pool_5m_hash_rate: f64,
            pool_60m_hash_rate: f64,
            pool_24h_hash_rate: f64,
            update_ts: u64,
            blocks: HashMap<String, Block>,
            fpps_rate: f64,
        }

        let helper: Helper = Helper::deserialize(deserializer)?;

        Ok(Self {
            pool_5m_hash_rate: HashRate::new(helper.hash_rate_unit, helper.pool_5m_hash_rate),
            pool_60m_hash_rate: HashRate::new(helper.hash_rate_unit, helper.pool_60m_hash_rate),
            pool_24h_hash_rate: HashRate::new(helper.hash_rate_unit, helper.pool_24h_hash_rate),
            update_ts: helper.update_ts,
            blocks: helper.blocks,
            fpps_rate: helper.fpps_rate,
        })
    }
}

/// User profile
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct UserProfile {
    /// Cumulative all-time reward
    pub all_time_reward: f64,
    /// Average hash rate for the last 5 minutes
    pub hash_rate_5m: HashRate,
    /// Average hash rate for the last 60 minutes
    pub hash_rate_60m: HashRate,
    /// Average hash rate for the last 24 hours
    pub hash_rate_24h: HashRate,
    /// Average hash rate for the previous UTC day
    pub hash_rate_yesterday: HashRate,
    /// Number of workers with `low` state
    pub low_workers: u32,
    /// Number of workers with `off` state
    pub off_workers: u32,
    /// Number of workers with `ok` state
    pub ok_workers: u32,
    /// Number of workers with disabled monitoring
    pub dis_workers: u32,
    /// Current reward balance
    pub current_balance: f64,
    /// Confirmed reward for this day
    pub today_reward: f64,
    /// Estimated reward for the current block
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

impl<'de> Deserialize<'de> for UserProfile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            #[serde(deserialize_with = "deserialize_number_from_string")]
            all_time_reward: f64,
            hash_rate_unit: HashRateUnit,
            hash_rate_5m: f64,
            hash_rate_60m: f64,
            hash_rate_24h: f64,
            hash_rate_yesterday: f64,
            low_workers: u32,
            off_workers: u32,
            ok_workers: u32,
            dis_workers: u32,
            #[serde(deserialize_with = "deserialize_number_from_string")]
            current_balance: f64,
            #[serde(deserialize_with = "deserialize_number_from_string")]
            today_reward: f64,
            #[serde(deserialize_with = "deserialize_number_from_string")]
            estimated_reward: f64,
            shares_5m: u32,
            shares_60m: u32,
            shares_24h: u32,
            shares_yesterday: u32,
        }

        let helper: Helper = Helper::deserialize(deserializer)?;

        Ok(Self {
            all_time_reward: helper.all_time_reward,
            hash_rate_5m: HashRate::new(helper.hash_rate_unit, helper.hash_rate_5m),
            hash_rate_60m: HashRate::new(helper.hash_rate_unit, helper.hash_rate_60m),
            hash_rate_24h: HashRate::new(helper.hash_rate_unit, helper.hash_rate_24h),
            hash_rate_yesterday: HashRate::new(helper.hash_rate_unit, helper.hash_rate_yesterday),
            low_workers: helper.low_workers,
            off_workers: helper.off_workers,
            ok_workers: helper.ok_workers,
            dis_workers: helper.dis_workers,
            current_balance: helper.current_balance,
            today_reward: helper.today_reward,
            estimated_reward: helper.estimated_reward,
            shares_5m: helper.shares_5m,
            shares_60m: helper.shares_60m,
            shares_24h: helper.shares_24h,
            shares_yesterday: helper.shares_yesterday,
        })
    }
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
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Worker {
    /// State of the worker (`ok`/`low`/`off`/`dis`)
    pub state: String,
    /// Unix time of the last accepted share
    pub last_share: u64,
    /// Current scoring hash rate
    pub hash_rate_scoring: HashRate,
    /// Average hash rate for the last 5 minutes
    pub hash_rate_5m: HashRate,
    /// Average hash rate for the last 60 minutes
    pub hash_rate_60m: HashRate,
    /// Average hash rate for the last 24 hours
    pub hash_rate_24h: HashRate,
    /// Active shares for last 5 minutes
    pub shares_5m: u64,
    /// Active shares for last 60 minutes
    pub shares_60m: u64,
    /// Active shares for last 24 hours
    pub shares_24h: u64,
}

impl<'de> Deserialize<'de> for Worker {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            state: String,
            last_share: u64,
            hash_rate_unit: HashRateUnit,
            hash_rate_scoring: f64,
            hash_rate_5m: f64,
            hash_rate_60m: f64,
            hash_rate_24h: f64,
            shares_5m: u64,
            shares_60m: u64,
            shares_24h: u64,
        }

        let helper: Helper = Helper::deserialize(deserializer)?;

        Ok(Self {
            state: helper.state,
            last_share: helper.last_share,
            hash_rate_scoring: HashRate::new(helper.hash_rate_unit, helper.hash_rate_scoring),
            hash_rate_5m: HashRate::new(helper.hash_rate_unit, helper.hash_rate_5m),
            hash_rate_60m: HashRate::new(helper.hash_rate_unit, helper.hash_rate_60m),
            hash_rate_24h: HashRate::new(helper.hash_rate_unit, helper.hash_rate_24h),
            shares_5m: helper.shares_5m,
            shares_60m: helper.shares_60m,
            shares_24h: helper.shares_24h,
        })
    }
}

/// Workers
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Workers {
    /// Workers
    pub workers: HashMap<String, Worker>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_stats_deserialization() {
        let json = r#"{
    "btc": {
        "hash_rate_unit": "Gh/s",
        "pool_active_workers": 1,
        "pool_5m_hash_rate": 5727000000.746604,
        "pool_60m_hash_rate": 5617000000.99422,
        "pool_24h_hash_rate": 5517000000.88519,
        "update_ts": 1699938300,
        "blocks": {
            "549753": {
                "date_found": 1542002919,
                "mining_duration": 3423,
                "total_shares": 4640771710739,
                "state": "confirmed",
                "confirmations_left": 0,
                "value": "12.92594863",
                "user_reward": "0.00006194",
                "pool_scoring_hash_rate": 5878745444.967269
            }
        },
        "fpps_rate": 0.00000241
    }
}"#;
        let user_profile: BtcResponse<PoolStats> = serde_json::from_str(json).unwrap();
        assert_eq!(
            user_profile.btc,
            PoolStats {
                pool_5m_hash_rate: HashRate::new(HashRateUnit::GH, 5727000000.746604),
                pool_60m_hash_rate: HashRate::new(HashRateUnit::GH, 5617000000.99422),
                pool_24h_hash_rate: HashRate::new(HashRateUnit::GH, 5517000000.88519),
                update_ts: 1699938300,
                blocks: HashMap::from([(
                    String::from("549753"),
                    Block {
                        date_found: 1542002919,
                        mining_duration: 3423,
                        total_shares: 4640771710739,
                        state: String::from("confirmed"),
                        confirmations_left: 0,
                        value: 12.92594863,
                        user_reward: 0.00006194,
                        pool_scoring_hash_rate: 5878745444.967269
                    }
                )]),
                fpps_rate: 0.00000241
            }
        );
    }

    #[test]
    fn test_user_profile_deserialization() {
        let json = r#"{
    "username": "username",
    "btc": {
        "all_time_reward": "0.15000000",
        "hash_rate_unit": "Gh/s",
        "hash_rate_5m": 27978,
        "hash_rate_60m": 28191,
        "hash_rate_24h": 28357,
        "hash_rate_yesterday": 28197,
        "low_workers": 0,
        "off_workers": 0,
        "ok_workers": 2,
        "dis_workers": 2,
        "current_balance": "0.15000000",
        "today_reward": "0.000166667",
        "estimated_reward": "0.00011940",
        "shares_5m": 123,
        "shares_60m": 1476,
        "shares_24h": 35424,
        "shares_yesterday": 0
    }
}"#;
        let user_profile: BtcResponse<UserProfile> = serde_json::from_str(json).unwrap();
        assert_eq!(
            user_profile.btc,
            UserProfile {
                all_time_reward: 0.15,
                hash_rate_5m: HashRate::new(HashRateUnit::GH, 27978.0),
                hash_rate_60m: HashRate::new(HashRateUnit::GH, 28191.0),
                hash_rate_24h: HashRate::new(HashRateUnit::GH, 28357.0),
                hash_rate_yesterday: HashRate::new(HashRateUnit::GH, 28197.0),
                low_workers: 0,
                off_workers: 0,
                ok_workers: 2,
                dis_workers: 2,
                current_balance: 0.15,
                today_reward: 0.000166667,
                estimated_reward: 0.00011940,
                shares_5m: 123,
                shares_60m: 1476,
                shares_24h: 35424,
                shares_yesterday: 0
            }
        );
    }

    #[test]
    fn test_workers_deserialization() {
        let json = r#"{
    "btc": {
        "workers": {
            "username.worker1": {
                "state": "ok",
                "last_share": 1542103204,
                "hash_rate_unit": "Gh/s",
                "hash_rate_scoring": 15342,
                "hash_rate_5m": 14977,
                "hash_rate_60m": 15302,
                "hash_rate_24h": 15351,
                "shares_5m": 90304,
                "shares_60m": 1125762,
                "shares_24h": 20945364

            },
            "username.worker2": {
                "state": "ok",
                "last_share": 1542103200,
                "hash_rate_unit": "Gh/s",
                "hash_rate_scoring": 12952,
                "hash_rate_5m": 13001,
                "hash_rate_60m": 12889,
                "hash_rate_24h": 13006,
                "shares_5m": 90304,
                "shares_60m": 1125762,
                "shares_24h": 20945364
            }
        }
    }
}"#;
        let user_profile: BtcResponse<Workers> = serde_json::from_str(json).unwrap();
        assert_eq!(
            user_profile.btc,
            Workers {
                workers: HashMap::from([
                    (
                        String::from("username.worker1"),
                        Worker {
                            state: String::from("ok"),
                            last_share: 1542103204,
                            hash_rate_scoring: HashRate::new(HashRateUnit::GH, 15342.0),
                            hash_rate_5m: HashRate::new(HashRateUnit::GH, 14977.0),
                            hash_rate_60m: HashRate::new(HashRateUnit::GH, 15302.0),
                            hash_rate_24h: HashRate::new(HashRateUnit::GH, 15351.0),
                            shares_5m: 90304,
                            shares_60m: 1125762,
                            shares_24h: 20945364
                        }
                    ),
                    (
                        String::from("username.worker2"),
                        Worker {
                            state: String::from("ok"),
                            last_share: 1542103200,
                            hash_rate_scoring: HashRate::new(HashRateUnit::GH, 12952.0),
                            hash_rate_5m: HashRate::new(HashRateUnit::GH, 13001.0),
                            hash_rate_60m: HashRate::new(HashRateUnit::GH, 12889.0),
                            hash_rate_24h: HashRate::new(HashRateUnit::GH, 13006.0),
                            shares_5m: 90304,
                            shares_60m: 1125762,
                            shares_24h: 20945364
                        }
                    )
                ])
            }
        );
    }
}
