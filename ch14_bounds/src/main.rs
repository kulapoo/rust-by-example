use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.length
    }
}

#[allow(dead_code)]
struct Triangle  { length: f64, height: f64 }

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    let rect = Rectangle { length: 5.1, height: 8.0 };

    print_debug(&rect);

    println!("{}", area(&rect))
}