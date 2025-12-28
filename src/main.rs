use ignore::WalkBuilder;
use std::fs::File;
use std::io::{self,Write};
use std::path::Path;
use std::path::PathBuf;

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

fn mergeFiles(files: &[std::path::PathBuf], output: &Path) -> io::Result<()>{
    let mut output = File::create(output)?;

    for path in files{
        if let Ok(content) = std::fs::read_to_string(path){
            writeln!(output, "[ {} ]", path.display())?;
            writeln!(output, "{}", content)?;
            writeln!(output)?;
        }
    }

    Ok(())
}

fn main(){
    let files = scanFiles("/Users/astra.celestine");
    println!("{:?}", &files);

    mergeFiles(&files, &Path::new("merged.txt")).unwrap();
}