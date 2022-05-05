#[path = "./Game/Gamble.rs"]
mod gamble;

#[path = "./common-concepts/common.rs"]
mod common;

#[path = "./ownership/ownership.rs"]
mod ownership;

#[path = "./structs/structs.rs"]
pub mod structs;

#[path = "./enum/enum.rs"]
pub mod enum_type;

#[path = "./error-handling/error-handling.rs"]
pub mod error_handling;

#[path = "./generic/generic.rs"]
pub mod generic;

fn main() {
    // Capter 2: Gamble game
    gamble::Gamble();

    // Chapter 3: Common programming concepts

    {
        common::common();
        println!("Max energy: {}", common::MAX_ENERGY);
        common::overflow();
        common::tup();
        common::array();
    }

    // 4. Ownnership
    {
        println!("\nOwnnership!");
        ownership::ownership();
    }

    // Can not compile these lines below
    let s1 = String::from("hello");
    let s2 = s1;
    // S1 no longer valid here
    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    // Deep copy
    let s1 = String::from("Good morning!");
    let s2 = s1.clone();
    println!("\nDeep clone s1: {}, s2: {}", s1, s2);

    // Ownership and function
    let some_string = String::from("King");
    ownership::takes_ownership(some_string);
    //some_string no longer valid here
    // println!("{}", some_string);

    let some_integer: i32 = 13;
    ownership::makes_copy(some_integer);
    println!("\n{}", some_integer);

    // 5. Structs
    let rust =
        structs::build_programming_language(String::from("Rust"), String::from("Graydon Hoare"));
    println!("\nRust: {}", rust);
    let c_plus_plus = structs::build_programming_language_clone(
        String::from("C++"),
        String::from("Bjarne Stroustrup"),
        rust,
    );
    println!("\nC++: {}", c_plus_plus);

    // Tuple struct
    let point = structs::Point::new(1, 3, 4);
    println!("\nTuple struct: {}", point);

    // Enum
    let ip_address = enum_type::IpAddr::V4(enum_type::Ip::new(String::from("127.0.0.1")));
    println!("{}", ip_address);

    let coin = enum_type::Coin::DOT(enum_type::Currency::VND);
    let price = enum_type::Coin::get_ticket(coin);

    println!("Price: {}", price);

    // Error handling
    error_handling::read_file();

    // Generic type
    let numbers = vec![20, 2, 14, 90, 49];
    let largest_num = generic::largest(&numbers);
    println!("The largest number: {}", largest_num);

    let characters = vec!['e', '€', '0', 'x', '1'];
    let largest_char = generic::largest(&characters);
    println!("The largest character: {}", largest_char);
    

    // let point1 = generic::Point::new(4, 1.4);
    // let point2 = generic::Point::new(6.5, 2.5);


}
