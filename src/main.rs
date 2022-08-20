use eyre::{eyre, Result};
use globset::{Glob, GlobSetBuilder};
use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
};
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
    let ignore_glob = builder.build()?;

    let target_glob = Glob::new("**/*.{ts,tsx}")?.compile_matcher();

    let mut writer = BufWriter::new(File::create("results.txt")?);

    let files = WalkDir::new(directory)
        .into_iter()
        .filter_entry(|e| !ignore_glob.is_match(e.path()))
        .filter_map(Result::ok)
        .filter(|e| target_glob.is_match(e.path()));
    for file in files {
        writeln!(writer, "{}", file.path().display())?;
    }

    Ok(())
}
