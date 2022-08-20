use eyre::{eyre, Result};
use globset::{Glob, GlobSetBuilder};
use regex::Regex;
use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
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

    let mut writer = BufWriter::new(File::create("translations.txt")?);

    let files = WalkDir::new(directory)
        .into_iter()
        .filter_entry(|e| !ignore_glob.is_match(e.path()))
        .filter_map(Result::ok)
        .filter(|e| target_glob.is_match(e.path()));

    let translation_regex = Regex::new(r#"__\("([^"]*)""#)?;
    for file in files {
        for translation in find_matches(file.path(), &translation_regex)? {
            writeln!(writer, "{translation}")?;
        }
    }

    Ok(())
}

fn find_matches(file: &Path, regex: &Regex) -> Result<Vec<String>> {
    let contents = fs::read_to_string(file)?;
    let matches: Vec<String> = regex
        .captures_iter(&contents)
        .map(|captures| captures[1].to_string())
        .collect();

    Ok(matches)
}
