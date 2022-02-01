use std::{env, fs};

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

pub fn print_begin_execution_msg(config: &Config) {
    eprintln!(
        "Searching for `{query}` in `{filename}`.\n",
        query = config.query,
        filename = config.filename
    );
}

impl Config {
    // TODO(sagunb): Optimize this w.r.t. Config creation and string cloning.
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(format!(
                "Insufficient arguments. Expected 2 but received {}.",
                args.len() - 1
            ));
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), String> {
    let contents =
        fs::read_to_string(config.filename).map_err(|e| format!("Error reading file: {}", e))?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;
