use std::fs;
use Mode::*;

#[derive(Debug)]
enum Mode {
    CASE_SENSITIVE,
    CASE_INSENSITIVE,
    REGEX,
}

#[derive(Debug)]
pub struct Config {
    query: String,
    file: String,
    mode: Mode,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let str_mode = args.get(3).expect("Error parsing config").as_str();
        if !vec!["case_insensitive", "case_sensitive", "regex"].contains(&str_mode) {
            return Err("Invalid Mode");
        }

        Ok(Config {
            query: args.get(1).expect("Error parsing config").clone(),
            file: args.get(2).expect("Error parsing config").clone(),
            mode: match args.get(3).expect("Error parsing config").as_str() {
                "case_insensitive" => Mode::CASE_INSENSITIVE,
                "case_sensitive" => Mode::CASE_SENSITIVE,
                "regex" => Mode::REGEX,
                _ => panic!("Invalid code reached"),
            },
        })
    }
}

fn search_case_sensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let mut v: Vec<&str> = Vec::new();

    for l in text.lines() {
        if l.contains(query) {
            v.push(l);
        }
    }

    return v;
}

fn search_case_insensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let q: String = query.to_lowercase();
    let mut v: Vec<&str> = Vec::new();

    for l in text.lines() {
        if l.to_lowercase().contains(&q) {
            v.push(l);
        }
    }

    return v;
}

fn search_regex<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let expr = regex::Regex::new(query).unwrap_or_else(|err| panic!("{}", err));

    let mut v: Vec<&str> = Vec::new();

    for l in text.lines() {
        if expr.is_match(l) {
            v.push(l);
        }
    }

    v
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(config.file)?;

    let results: Vec<&str> = match config.mode {
        CASE_INSENSITIVE => search_case_insensitive(&config.query, &text),
        CASE_SENSITIVE => search_case_sensitive(&config.query, &text),
        REGEX => search_regex(&config.query, &text),
    };

    for l in results {
        println!("{}", l);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "test";
        let text = "\
a;slkdjf
a;lsdkfj
abc test query
a;sldkfj
a;sldkfj
        ";
        assert_eq!(vec!["abc test query"], search_case_sensitive(query, text));
    }

    #[test]
    fn case_insensitive() {
        let query = "test";
        let text = "\
a;slkdjf
a;lsdkfj
abc TEST query
a;sldkfj
a;sldkfj
        ";

        assert_eq!(vec!["abc TEST query"], search_case_insensitive(query, text));
    }

    #[test]
    fn test_regex() {
        let query = r"abc.*query";
        let text = "\
a;slkdjf
a;lsdkfj
abc TEST query
a;sldkfj
a;sldkfj
        ";

        assert_eq!(vec!["abc TEST query"], search_regex(query, text));
    }
}
