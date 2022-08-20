use eyre::{eyre, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use regex::Regex;
use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};
use walkdir::WalkDir;

const IGNORE: [&str; 4] = [
    "**/node_modules",
    "**/dist",
    "**/global.type.ts",
    "**/applications/shop-e2e",
];
const SOURCE_FILES: &str = "**/*.{ts,tsx}";

fn main() -> Result<()> {
    let directory = env::args()
        .nth(1)
        .ok_or_else(|| eyre!("Missing argument target directory"))?;

    let ignore_glob = build_globset(&IGNORE)?;
    let target_glob = Glob::new(SOURCE_FILES)?.compile_matcher();
    let files = WalkDir::new(directory)
        .into_iter()
        .filter_entry(|dir_entry| !ignore_glob.is_match(dir_entry.path()))
        .filter_map(Result::ok)
        .filter(|dir_entry| target_glob.is_match(dir_entry.path()));

    let translation_regex = Regex::new(r#"__\("([^"]*)""#)?;
    let mut result_writer = BufWriter::new(File::create("translations.txt")?);
    for file in files {
        for translation in find_matches(file.path(), &translation_regex)? {
            writeln!(result_writer, "{translation}")?;
        }
    }

    Ok(())
}

fn build_globset(patterns: &[&str]) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        builder.add(Glob::new(pattern)?);
    }

    Ok(builder.build()?)
}

fn find_matches(file: &Path, regex: &Regex) -> Result<Vec<String>> {
    let contents = fs::read_to_string(file)?;
    let matches: Vec<String> = regex
        .captures_iter(&contents)
        .map(|captures| captures[1].to_string())
        .collect();

    Ok(matches)
}
