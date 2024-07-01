use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use clap::clap_derive::*;
use clap::{arg, Parser};
use grep::matcher::Matcher;
use grep::regex::RegexMatcher;
use grep::searcher::sinks::UTF8;
use grep::searcher::Searcher;
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Clone, ValueEnum)]
#[value(rename_all = "kebab-case")]
enum Preset {
    JS,
    Python,
    Rust,
    Go,
}

#[derive(Debug, Parser)]
#[command(
about = "Tool to grab (yoink) env variables from a workspace into env example file",
version = env!("CARGO_PKG_VERSION")
)]
struct Cli {
    /// Workspace Directory
    #[arg()]
    workspace_directory: String,

    /// Output Directory
    #[arg(short = 'o', long = "out", default_value = "./")]
    output_directory: String,

    /// Env example file
    #[arg(short = 'e', long, default_value = ".env.example")]
    example_file_name: String,

    /// Custom Regex pattern to capture the env variables
    #[arg(
        short = 'x',
        long,
        required_unless_present("preset"),
        conflicts_with = "preset"
    )]
    regex_pattern: Option<String>,

    /// Use from the list of regex presets based on the language
    #[arg(
        short = 'p',
        long,
        required_unless_present("regex_pattern"),
        conflicts_with = "regex_pattern"
    )]
    preset: Option<Preset>,
}

fn main() {
    let args = Cli::parse();

    let output_directory = args.output_directory;
    let example_file_name = args.example_file_name;
    let workspace_directory = args.workspace_directory;
    let regex_pattern = args.regex_pattern;
    let preset = args.preset;

    let files = list_files(&workspace_directory);

    let regex_sets = if regex_pattern.is_some() {
        vec![regex_pattern.unwrap()]
    } else {
        get_preset_regex_pattern(preset.expect("No preset selected!"))
    };

    let mut env_variables: Vec<String> = Vec::new();

    for regex in regex_sets {
        env_variables.extend(fetch_env_variables(&files, regex));
    }

    let unique_env_variables: HashSet<String> = env_variables.into_iter().collect();

    let mut output = String::from("");
    for env in unique_env_variables {
        output += &env;
        output += "=\n";
    }
    let mut path = PathBuf::from(output_directory);
    path.extend(&[example_file_name]);

    fs::write(path, output).expect("Failed to write to file....");
}

fn fetch_env_variables(files: &Vec<DirEntry>, regex_pattern: String) -> Vec<String> {
    let mut env_variables: Vec<String> = Vec::new();
    for file in files {
        env_variables.extend(extract_env_variables(regex_pattern.clone(), &file).unwrap());
    }
    return env_variables;
}

fn directory_exists(directory: &String) -> bool {
    return Path::new(directory.as_str()).is_dir();
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn list_files(directory: &String) -> Vec<DirEntry> {
    let mut files = Vec::new();
    if !directory_exists(directory) {
        eprintln!("Directory {} not found", directory);
    }
    let walker = WalkDir::new(directory)
        .into_iter()
        .filter_entry(|file| !is_hidden(file))
        .filter_map(|file| file.ok());

    for file in walker {
        if file.metadata().unwrap().is_file() {
            files.push(file);
        }
    }

    return files;
}

fn extract_env_variables(pattern: String, dir: &DirEntry) -> Result<Vec<String>, Box<dyn Error>> {
    let matcher = RegexMatcher::new(pattern.as_str())?;

    let mut matches = vec![];

    let mut searcher = Searcher::new();
    let re = Regex::new(pattern.as_str()).unwrap();

    searcher.search_path(
        &matcher,
        dir.path(),
        UTF8(|_lnum, line| {
            let mymatches = matcher.find(line.as_bytes())?.unwrap();
            if let Some(capture) = re.captures(&line[mymatches]) {
                let extracted_text = capture.get(1).unwrap().as_str().trim().to_string();
                matches.push(extracted_text);
            }

            Ok(true)
        }),
    )?;

    Ok(matches)
}

fn get_preset_regex_pattern(preset: Preset) -> Vec<String> {
    match preset {
        Preset::JS => vec![
            String::from(r"process\.env\.([a-zA-Z_][a-zA-Z0-9_]*)\b"),
            String::from(r#"process\.env\[['"]([^'"]+)['"]\]"#),
        ],
        Preset::Go => vec![String::from(r#"os\.Getenv\(["']([^"']+)["']\)"#)],
        Preset::Python => vec![
            String::from(r#"os\.environ\[['"]([^'"]+)['"]\]"#),
            String::from(r#"os\.environ\.get\(['"]([^'"]+)['"]\)"#),
        ],
        Preset::Rust => vec![String::from(r#"env::var\(["']([^"']+)["']\)"#)],
    }
}
