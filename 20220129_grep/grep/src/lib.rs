use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;
    let case_insensitive = env::var("CASE_INSENSITIVE").is_ok();
    for line in search(&config.query, &contents, case_insensitive) {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            Err(String::from("not enough arguments!"))
        } else {
            Ok(Config { query: args[1].to_string(), file_name: args[2].to_string()})
        }
    }
}

fn search<'a>(query: &str, contents: &'a str, case_insensitive: bool) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    let query = if case_insensitive {
         query.to_lowercase()
    } else {
        query.to_string()
    };
    for line in contents.lines() {
        let original_line = line;
        let line = if case_insensitive {
            line.to_lowercase()
        } else {
            line.to_string()
        };
        if line.contains(&query) {
            results.push(&original_line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_result() {
        let query = "book";
        let contents = "\
I like it,
it's a nice book,
do you like this Book?
        ";
        assert_eq!(vec!["it's a nice book,"], search(query, contents, false));
        assert_eq!(vec!["it's a nice book,", "do you like this Book?"], search(query, contents, true));
    }
}