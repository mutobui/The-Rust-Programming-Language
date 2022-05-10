use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    query: String,
    file_name: String,
    is_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get any query string"),
        };
        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("File name not given"),
        };
        

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
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
    contents.lines().filter(|line| {
        if is_sensitive == true{
            line.to_lowercase().contains(&query.to_lowercase())
        } else {
            line.contains(query)
        }
    }).collect()
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
