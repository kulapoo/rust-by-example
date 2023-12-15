use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

impl <T> PrintInOption for T where
    Option<T>: Debug {
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

use std::fmt::Display;

fn displayable<T: Display>(t: T) -> impl Display { t }



fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
