use chrono::prelude::*;
use std::fmt;

pub struct ProgrammingLanguage {
    name: String,
    author: String,
    release_date: DateTime<Local>,
    description: String,
}

pub fn build_programming_language(name: String, author: String) -> ProgrammingLanguage {
    ProgrammingLanguage {
        name,
        author,
        release_date: Local::now(),
        description: String::from("This is the best programming language ever have!"),
    }
}

pub fn build_programming_language_clone(name: String, author: String, clone: ProgrammingLanguage) -> ProgrammingLanguage{
    ProgrammingLanguage {
        name,
        author,
        ..clone
    }
}

impl fmt::Display for ProgrammingLanguage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "\nName: {}", self.name);
        write!(f, "\nAuthor: {}", self.author);
        write!(f, "\nRelease: {}", self.release_date);
        write!(f, "\nAbout: {}", self.description)

    }
}


pub struct Point(i32, i32, i32);

impl Point {
    pub fn new(first: i32, second: i32, third: i32) -> Point {
        Point(first, second, third)
    }
}

impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "\nFirst: {}", self.0);
            write!(f, "\nSecond: {}", self.1);
            write!(f, "\nThird: {}", self.2)
        }
}
