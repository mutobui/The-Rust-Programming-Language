use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    query: String,
    file_name: String,
    is_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("expected more than 2 arguments");
        }
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        let query = &args[1];
        let file_name = &args[2];
        Ok(Config {
            query: query.to_string(),
            file_name: file_name.to_string(),
            is_sensitive: case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;
    for line in search(&config.query, &contents, config.is_sensitive) {
        println!("{:?}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, is_sensitive: bool) -> Vec<&'a str> {
    let mut result: Vec<&str> = vec![];
    let mut query: String = query.to_string();
    if is_sensitive == true {
        query = query.to_lowercase();
    }
    for line in contents.lines() {
        if is_sensitive == true {
            if line.to_lowercase().contains(&query) {
                result.push(line);
            }
        } else {
            if line.contains(&query) {
                result.push(line);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn search_sensitive() {
        let content = "\
I love those who can smile in trouble,
who can gather strength from distress,
and grow brave by reflection.
'Tis the business of little minds to shrink,
but they whose heart is firm,
and whose conscience approves their conduct,
will pursue their principles unto death.";

        let query = "brave";
        assert_eq!(
            search(query, content, false),
            ["and grow brave by reflection."]
        );
    }

    #[test]
    fn search_insensitive() {
        let content = "\
I love those who can smile in trouble,
who can gather strength from distress,
and grow brave by reflection.
'Tis the business of little minds to shrink,
but they whose heart is firm,
and whose conscience approves their conduct,
will pursue their principles unto death.";

        let query = "Brave";
        assert_eq!(
            search(query, content, true),
            ["and grow brave by reflection."]
        );

        let query = "Brave";
        assert_eq!(search(query, content, false), Vec::<&str>::new());
    }
}
