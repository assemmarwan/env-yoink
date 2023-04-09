use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::{fs, io};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use clap::{arg, Parser};
use grep::matcher::Matcher;
use walkdir::{DirEntry, WalkDir};
use grep::regex::{RegexCaptures, RegexMatcher, RegexMatcherBuilder};
use grep::searcher::Searcher;
use grep::searcher::sinks::UTF8;
use regex::{escape, Regex};


#[derive(Parser, Debug)]
#[command(
about = "Tool to grab (yoink) env variables from a workspace into env example file",
version = env ! ("CARGO_PKG_VERSION")
)]
struct Args {
    /// Env capture pattern
    #[arg(short = 'p', long = "pattern", help = "Pattern to capture env variables ")]
    env_capture_pattern: String,

    /// Output Directory
    #[arg(short = 'o', long = "out", default_value = "./")]
    output_directory: String,

    /// Env example file
    #[arg(short = 'e', long, default_value = ".env.example")]
    example_file_name: String,

    /// Workspace Directory
    #[arg(short = 'd', long, default_value = "./")]
    workspace_directory: String,
}

fn main() {
    let args = Args::parse();
    let env_capture_pattern = args.env_capture_pattern;
    let output_directory = args.output_directory;
    let example_file_name = args.example_file_name;
    let workspace_directory = args.workspace_directory;


    let files = list_files(&workspace_directory);
    let mut env_variables = vec![];
    for file in files {
        env_variables.extend(extract_env_variables(env_capture_pattern.clone(), &file).unwrap());
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

fn directory_exists(directory: &String) -> bool {
    return Path::new(directory.as_str()).is_dir();
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn is_file(entry: &DirEntry) -> bool {
    entry.metadata().unwrap().is_file()
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


    searcher.search_path(&matcher, dir.path(), UTF8(|lnum, line| {
        let mymatches = matcher.find(line.as_bytes())?.unwrap();
        if let Some(capture) = re.captures(&line[mymatches]) {
            let extracted_text = capture.get(1).unwrap().as_str().trim().to_string();
            matches.push(extracted_text);
        }

        Ok(true)
    }))?;

    Ok(matches)
}



