# Braiins Pool API

## Description

[Braiins Pool](https://braiins.com) client.

## Getting started

```rust,no_run
use braiinspool::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Construct client
    let client = BraiinsPoolClient::new("apikey")?;

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

## Minimum Supported Rust Version (MSRV)

This project is built with the Rust language version `2024` and requires a minimum compiler version of `1.85.0`.

## License

This project is distributed under the MIT software license - see the [LICENSE](./LICENSE) file for details
