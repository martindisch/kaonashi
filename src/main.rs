use eyre::{eyre, Result};
use std::env;

fn main() -> Result<()> {
    let directory = env::args()
        .nth(1)
        .ok_or_else(|| eyre!("Missing argument target directory"))?;

    let directory = format!("{directory}/**/*.tsx");
    println!("{directory}");

    let files = glob::glob(&directory)?.filter_map(Result::ok);
    for file in files {
        println!("{}", file.display());
    }

    Ok(())
}
