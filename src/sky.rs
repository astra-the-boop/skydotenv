use bsky_sdk::BskyAgent;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use atrium_api::app::bsky::feed::post::RecordData;
use atrium_api::types::string::Datetime;

async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let appPass = env::var("APP_PASS")?;
    let username = env::var("USERNAME")?;

    let agent = BskyAgent::builder().build().await?;

    agent.login(&username, &appPass).await?;

    let post = RecordData{
        created_at: Datetime::now(),
        embed: None,
        entities: None,
        facets: None,
        labels: None,
        langs: None,
        reply: None,
        tags: None,
        text: "Hello, world! (again)".to_string(),
    };

    agent.create_record(post).await?;

    Ok(())
}