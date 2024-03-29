use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use clap::{arg, Parser};
use clap::clap_derive::*;
use grep::matcher::Matcher;
use grep::regex::RegexMatcher;
use grep::searcher::Searcher;
use grep::searcher::sinks::UTF8;
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


#[derive(Debug, Clone, ValueEnum)]
#[value(rename_all = "kebab-case")]
enum Mode {
    Regex,
    Preset,
}


#[derive(Debug, Parser)]
#[command(
about = "Tool to grab (yoink) env variables from a workspace into env example file",
version = env!("CARGO_PKG_VERSION")
)]
struct Cli {
    /// Output Directory
    #[arg(short = 'o', long = "out", default_value = "./")]
    output_directory: String,

    /// Env example file
    #[arg(short = 'e', long, default_value = ".env.example")]
    example_file_name: String,

    /// Workspace Directory
    #[arg(short = 'd', long, default_value = "./")]
    workspace_directory: String,

    #[arg(short, long)]
    mode: Mode,

    #[arg(short, long, required_if_eq("mode", "regex"))]
    regex_pattern: Option<String>,

    #[arg(short, long, required_if_eq("mode", "preset"))]
    preset: Option<Preset>,
}


fn main() {
    let args = Cli::parse();

    let output_directory = args.output_directory;
    let example_file_name = args.example_file_name;
    let workspace_directory = args.workspace_directory;
    let mode = args.mode;
    let regex_pattern = args.regex_pattern;
    let preset = args.preset;

    let files = list_files(&workspace_directory);

    let regex_sets = match mode {
        Mode::Regex => match regex_pattern {
            Some(value) => vec![value],
            None => panic!("Invalid Regex")
        },
        Mode::Preset => match preset {
            Some(value) => get_preset_regex_pattern(value),
            None => panic!("Invalid Preset")
        },
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
    entry.file_name()
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


    searcher.search_path(&matcher, dir.path(), UTF8(|_lnum, line| {
        let mymatches = matcher.find(line.as_bytes())?.unwrap();
        if let Some(capture) = re.captures(&line[mymatches]) {
            let extracted_text = capture.get(1).unwrap().as_str().trim().to_string();
            matches.push(extracted_text);
        }

        Ok(true)
    }))?;

    Ok(matches)
}


fn print_command_args(args: Cli) {
    todo!("Print the arguments in a pretty fashion")
}


fn get_preset_regex_pattern(preset: Preset) -> Vec<String> {
    match preset {
        Preset::JS => vec![String::from(r"process\.env\.([a-zA-Z_][a-zA-Z0-9_]*)\b"),
                           String::from(r#"process\.env\[['"]([^'"]+)['"]\]"#)],
        Preset::Go => vec![String::from(r#"os\.Getenv\(["']([^"']+)["']\)"#)],
        Preset::Python => vec![String::from(r#"os\.environ\[['"]([^'"]+)['"]\]"#),
                               String::from(r#"os\.environ\.get\(['"]([^'"]+)['"]\)"#)],
        Preset::Rust => vec![String::from(r#"env::var\(["']([^"']+)["']\)"#)],
    }
}
