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
