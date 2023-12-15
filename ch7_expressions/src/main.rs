fn main() {
    // variable binding
    let x = 5u32;

    // expression;
    x;
    x + 1;
    15;

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };
    let a = {

    };
    println!("{}", y);

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
    println!("z is {:?}", a);
}