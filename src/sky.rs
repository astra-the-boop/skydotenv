use bsky_sdk::BskyAgent;
use chrono::Utc;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use atrium_api::app::bsky::feed::post::Record;

async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let appPass = env::var("APP_PASS")?;
    let username = env::var("USERNAME")?;

    let agent = BskyAgent::builder().build().await?;

    agent.login(&username, &appPass).await?;

    let post = Record{
        created_at: Utc::now().to_rfc3339(),
        embed: None,
        entities: None,
        facets: None,
        reply: None,
        text: "Hello, world!".to_string(),
    };

    agent.create_record(post).await?;

    Ok(());
}


// fn main(){
//     dotenv().ok();
//
//     let appPass = env::var("APP_PASS").expect("APP_PASS is not set");
//     let username = env::var("USERNAME").expect("USERNAME is not set");
// }