use eyre::{eyre, Result};
use std::env;
use walkdir::WalkDir;

fn main() -> Result<()> {
    let directory = env::args()
        .nth(1)
        .ok_or_else(|| eyre!("Missing argument target directory"))?;

    let files = WalkDir::new(directory).into_iter().filter_map(Result::ok);
    for file in files {
        println!("{}", file.path().display());
    }

    Ok(())
}
