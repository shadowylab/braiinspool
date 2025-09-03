# Braiins Pool API

## Description

[Braiins](https://braiins.com) client library to check miners status from [Rust](https://rust-lang.org).

## Example

```toml
braiinspool = "0.1"
tokio = { version = "1", features = ["full"] }
```

```rust,no_run
use braiinspool::model::{DailyRewards, PoolStats, UserProfile, Workers};
use braiinspool::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Init client
    let client = Client::new("apikey", Some("socks5h://127.0.0.1:9050"))?;

    // Check tor connection
    println!("{}", client.check_tor_connection().await?);

    // Get pool stats
    let pool_stats: PoolStats = client.pool_stats().await?;
    println!("{:#?}", pool_stats);

    // Get user profile
    let user_profile: UserProfile = client.user_profile().await?;
    println!("{:#?}", user_profile);

    // Get daily rewards
    let daily_rewards: DailyRewards = client.daily_rewards().await?;
    println!("{:#?}", daily_rewards);

    // Get workers
    let workers: Workers = client.workers().await?;
    println!("{:#?}", workers);

    Ok(())
}
```

## License

This project is distributed under the MIT software license - see the [LICENSE](./LICENSE) file for details
