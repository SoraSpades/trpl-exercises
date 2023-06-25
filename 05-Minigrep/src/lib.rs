use std::fs;
use Mode::*;

#[derive(Debug)]
enum Mode {
    CaseSensitive,
    CaseInsensitive,
    Regex,
}

#[derive(Debug)]
pub struct Config {
    query: String,
    file: String,
    mode: Mode,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => return Err("Didn't specify query")
        };

        let file = match args.next() {
            Some(f) => f,
            None => return Err("Didn't specify file")
        };

        let mode = match args.next() {
            Some(m) => match m.as_str() {
                "case_insensitive" => Mode::CaseInsensitive,
                "case_sensitive" => Mode::CaseSensitive,
                "regex" => Mode::Regex,
                _ => return Err("Invalid mode. Must be 'case_insensitive', 'case_sensitive' or 'regex'")
            }
            None => Mode::CaseInsensitive
        };

        Ok(Config { query, file, mode })
    }
}

fn search_case_sensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let mut v: Vec<&str> = Vec::new();

    for l in text.lines() {
        if l.contains(query) {
            v.push(l);
        }
    }

    v
}

fn search_case_insensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let q: String = query.to_lowercase();
    let mut v: Vec<&str> = Vec::new();

    for l in text.lines() {
        if l.to_lowercase().contains(&q) {
            v.push(l);
        }
    }

    v
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
        CaseInsensitive => search_case_insensitive(&config.query, &text),
        CaseSensitive => search_case_sensitive(&config.query, &text),
        Regex => search_regex(&config.query, &text),
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