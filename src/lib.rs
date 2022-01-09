use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        parse_args()
    }
}

pub fn parse_args() -> Result<Config, Box<dyn Error>> {
    let mut derived_args = env::args();

    if derived_args.len() < 3 {
        return Err("not enough arguments provided!".into());
    }

    derived_args.next();

    let query = match derived_args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a query string".into()),
    };

    let filename = match derived_args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a file name".into()),
    };

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config {
        query,
        filename,
        case_sensitive,
    })
}

pub fn get_file_contents(config: &Config) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    Ok(contents)
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let config = match parse_args() {
        Ok(config) => config,
        Err(err) => return Err(err),
    };

    let content = get_file_contents(&config)?;

    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
