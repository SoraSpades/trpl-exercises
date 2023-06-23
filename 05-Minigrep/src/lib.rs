use std::fs;

#[derive(Debug)]
pub struct Config {
    query: String,
    file: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Config {
            query: args.get(1).expect("Error parsing config").clone(),
            file: args.get(2).expect("Error parsing config").clone(),
            case_sensitive: !(std::env::var("IGNORE_CASE").is_ok())
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

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(config.file)?;

    let results: Vec<&str> = if config.case_sensitive {
        search_case_sensitive(&config.query, &text)
    } else {
        search_case_insensitive(&config.query, &text)
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
        let text  = "\
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
        let text  = "\
a;slkdjf
a;lsdkfj
abc TEST query
a;sldkfj
a;sldkfj
        ";

        assert_eq!(vec!["abc TEST query"], search_case_insensitive(query, text));
    }

}