use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: Option<Vec<String>>) -> Result<Config, Box<dyn Error>> {
        parse_args(args)
    }
}

pub fn parse_args(args: Option<Vec<String>>) -> Result<Config, Box<dyn Error>> {
    let derived_args: Vec<String> = if None == args {
        env::args().collect()
    } else {
        args.unwrap()
    };

    if derived_args.len() < 3 {
        return Err("not enough arguments provided!".into());
    }

    let query = derived_args[1].clone();
    let filename = derived_args[2].clone();

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
    let config = match parse_args(None) {
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
