pub const MAX_ENERGY: u128 = 100_000;

pub fn common() {
   println!("\nShadowing!");
   let x = 13;
   println!("With immutable x before: {}", x);

   let x = 15;
   println!("With immutable x now: {}", x);

   println!("\nChagne type with immutble variable!");

   let y = "Hard time create strong men";
   println!(" With immutable y before: {}", y);

   let y = y.len();
   println!(" With immutable y now: {}", y);
}

#[allow(overflowing_literals)]
pub fn overflow() {
   println!("\nOverflow!");

   let x: u8 = 300;

   println!("x: {}", x)
}

pub fn tup() {

   println!("\nThe Tuple Type");
   println!("A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    Tuples have a fixed length: once declared, they cannot grow or shrink in size.");
   let tup : (u8, String, bool) =(13, String::from("Hello"), true);

   println!("tup: {}", tup.1);

}

pub fn array() {
   println!("\n The Array Type");
   println!("Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type.
    Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.");

    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("array length: {}", a.len());
}
