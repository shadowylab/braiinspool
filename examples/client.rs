// Copyright (c) 2021-2022 Yuki Kishimoto
// Distributed under the MIT software license

extern crate braiinspool;

use braiinspool::Client;

#[tokio::main]
async fn main() {
    // Init client
    let client = Client::new("apikey", Some("socks5://127.0.0.1:9050")).unwrap();

    // Check tor connection
    println!("{:#?}", client.check_tor_connection().await.unwrap());

    // Get pool stats
    println!("{:#?}", client.pool_stats().await.unwrap());

    // Get user profile
    println!("{:#?}", client.user_profile().await.unwrap());

    // Get daily rewards
    println!("{:#?}", client.daily_rewards().await.unwrap());

    // Get workers
    println!("{:#?}", client.workers().await.unwrap());
}
