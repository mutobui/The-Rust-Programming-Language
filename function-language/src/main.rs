use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Cacher<T, F>
where
    F: Fn(T) -> T,
{
    calculation: F,
    values: HashMap<T, T>,
}

impl<T, F> Cacher<T, F>
where
    F: Fn(T) -> T,
    T: std::cmp::PartialEq,
    T: Eq + Hash + Copy,
{
    pub fn new(f: F) -> Self {
        Cacher {
            calculation: f,
            values: HashMap::new(),
        }
    }

    pub fn calculating(& mut self, n: T) -> T {
        if self.values.get(&n) == None {
            self.values.insert(n, (self.calculation)(n));
        }
        *self.values.get(&n).unwrap()
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num: u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.calculating(random_number));
        println!("Next, do {} situps!", expensive_result.calculating(random_number + 5));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!",expensive_result.calculating(random_number));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
