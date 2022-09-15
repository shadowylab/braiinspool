// Copyright (c) 2021-2022 Yuki Kishimoto
// Distributed under the MIT software license

use braiinspool::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Init client
    let client = Client::new("apikey", Some("socks5h://127.0.0.1:9050"))?;

    // Check tor connection
    println!("{}", client.check_tor_connection().await?);

    // Get pool stats
    println!("{:#?}", client.pool_stats().await?);

    // Get user profile
    println!("{:#?}", client.user_profile().await?);

    // Get daily rewards
    println!("{:#?}", client.daily_rewards().await?);

    // Get workers
    println!("{:#?}", client.workers().await?);

    Ok(())
}
