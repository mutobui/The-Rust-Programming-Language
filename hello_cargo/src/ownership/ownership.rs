

pub fn ownership () {

    let s1 = String::from("Good evening");
    let s2 = s1;

    println!("\nS2: {}", s2);
}

pub fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

pub fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

pub fn gives_ownership() -> String {
    let s = String::from("kids");
    s
}

pub fn takes_and_gives_back(some_string: String) -> String {
    some_string
}