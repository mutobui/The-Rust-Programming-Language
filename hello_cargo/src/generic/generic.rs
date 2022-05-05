use std::cmp::PartialOrd;

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    pub fn new(_x: T, _y: U) -> Point<T, U> {
        Point { x: _x, y: _y }
    }

    pub fn mixup(_other: Point<T, U>) -> Point<T, U> {
        Point {
            x: _other.x,
            y: _other.y,
        }
    }

}
