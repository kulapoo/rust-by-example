fn main() {
    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // println!("Hello, world!");
    println!("{number:>10}", number=1);
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
