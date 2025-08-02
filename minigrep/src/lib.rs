use std::env;
use anyhow::Result;
use std::fs;

#[derive(Debug)]
pub struct Config {
    search_string: String,
    file_name: String,
    print_color: bool,
    case_sensitive: bool,
}

impl Config {
    pub fn build(search_string: String, file_name: String, print_color: bool) -> Self {

        // if any value is set in env_var, we will be case insenstive
        let case_sensitive = env::var("MINIGREP_CASE_INSENSITIVE").is_err();

        Self {
            search_string,
            file_name,
            print_color,
            case_sensitive,
        }
    }
}

pub fn run(config: Config) ->  Result<()> {
    let contents = fs::read_to_string(config.file_name)?;
    let mut matches: Vec<&str> = vec![];
    if config.case_sensitive {
        matches = search(&config.search_string, &contents)
    }

    for match_s in matches {
        println!("{match_s}");
    }

    Ok(())
}

fn search<'a>(search_string: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(search_string) {
            results.push(line);
        }
    }

    results
}

// fn search_case_insensitve<'a>(search_string: &'a str, contents: &'a str) -> Vec<&'a str> {

// }