mod sky;

use bsky_sdk::BskyAgent;
use chrono::Utc;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use atrium_api::app::bsky::feed::post::RecordData;
use atrium_api::types::string::Datetime;

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
        text: "Hello, world!".to_string(),
    };

    agent.create_record(post).await?;

    Ok(())
}


// fn main(){
//     dotenv().ok();
//
//     let appPass = env::var("APP_PASS").expect("APP_PASS is not set");
//     let username = env::var("USERNAME").expect("USERNAME is not set");
// }


// mod sky;
//
// use ignore::WalkBuilder;
// use std::fs::File;
// use std::io::{self,Write};
// use std::path::Path;
// use std::path::PathBuf;
//
// fn scanFiles(root: &str) -> Vec<PathBuf>{
//     let mut files = Vec::new();
//
//     let walker = WalkBuilder::new(root).hidden(false).follow_links(false).git_exclude(false).git_global(false).git_ignore(false).build();
//
//     for item in walker{
//         if let Ok(item) = item{
//             let path = item.path();
//
//             if path.file_name().and_then(|n| n.to_str()) == Some(".env"){
//                 files.push(path.to_path_buf());
//             }
//         }
//     }
//
//     files
// }
//
// fn mergeFiles(files: &[std::path::PathBuf], output: &Path) -> io::Result<()>{
//     let mut output = File::create(output)?;
//
//     for path in files{
//         if let Ok(content) = std::fs::read_to_string(path){
//             writeln!(output, "[ {} ]", path.display())?;
//             writeln!(output, "{}", content)?;
//             writeln!(output)?;
//         }
//     }
//
//     Ok(())
// }
//
// fn main(){
//     let files = scanFiles("/Users/astra.celestine");
//     println!("{:?}", &files);
//
//     mergeFiles(&files, &Path::new("merged.txt")).unwrap();
// }