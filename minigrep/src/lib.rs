use std::{env};
use anyhow::Result;
use std::fs;
use owo_colors::OwoColorize;

#[derive(Debug)]
pub struct Config {
    pub search_string: String,
    pub file_name: String,
    pub do_print_color: bool,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(search_string: String, file_name: String, do_print_color: bool) -> Self {

        // if any value is set in env_var, we will be case insenstive
        let case_sensitive = env::var("MINIGREP_CASE_INSENSITIVE").is_err();

        Self {
            search_string,
            file_name,
            do_print_color,
            case_sensitive,
        }
    }
}

pub fn run(config: Config) ->  Result<()> {
    let contents = fs::read_to_string(&config.file_name)?;
    let mut _matches: Vec<&str> = vec![];
    if config.case_sensitive {
        _matches = search(&config.search_string, &contents);
    } else {
        _matches = search_case_insensitve(&config.search_string, &contents);
    }
    print_matches(&_matches, &config);
    Ok(())
}


fn search<'a>(search_string: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(search_string) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitve<'a>(search_string: &str, contents: &'a str) -> Vec<&'a str> {

    let case_insensitve_search_string = search_string.to_lowercase();
    search(&case_insensitve_search_string, contents)
}

fn print_matches(matches: &Vec<&str>, config: &Config) {
    if config.do_print_color {
        for _match in matches {
            _match.split_ascii_whitespace().for_each(|word|{
                if !word.contains(&config.search_string) {
                    print!("{word} ");
                } else {
                    print!("{} ", word.if_supports_color(owo_colors::Stream::Stdout, |word| word.red()));
                }
            });
            println!();
        }

    } else {
        for _match in matches {
            println!("{_match}");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let search_string = "are";
        let content = r#"Waffles are good
We are back
dont match this"#;

        assert_eq!(
            vec!["Waffles are good", "We are back"],
            search(search_string, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let search_string = "aRe";
        let content = r#"Waffles are good
We are back
dont match this"#;

        assert_eq!(
            vec!["Waffles are good", "We are back"],
            search_case_insensitve(search_string, content)
        );
    }


}