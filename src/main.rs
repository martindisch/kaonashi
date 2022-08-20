use eyre::{eyre, Result};
use std::env;

fn main() -> Result<()> {
    let directory = env::args()
        .nth(1)
        .ok_or_else(|| eyre!("Missing argument target directory"))?;

    println!("{directory}");

    Ok(())
}
