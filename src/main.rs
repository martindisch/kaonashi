use eyre::{eyre, Result};
use globset::{Glob, GlobSetBuilder};
use std::env;
use walkdir::WalkDir;

fn main() -> Result<()> {
    let directory = env::args()
        .nth(1)
        .ok_or_else(|| eyre!("Missing argument target directory"))?;

    let mut builder = GlobSetBuilder::new();
    builder.add(Glob::new("**/node_modules")?);
    builder.add(Glob::new("**/dist")?);
    builder.add(Glob::new("**/global.type.ts")?);
    builder.add(Glob::new("**/applications/shop-e2e")?);
    let ignore_set = builder.build()?;

    let files = WalkDir::new(directory)
        .into_iter()
        .filter_entry(|e| !ignore_set.is_match(e.path()))
        .filter_map(Result::ok);
    for file in files {
        println!("{}", file.path().display());
    }

    Ok(())
}
