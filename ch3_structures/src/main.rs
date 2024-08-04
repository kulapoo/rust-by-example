#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

fn react_area(rect: &Rectangle) -> f32 {
    (rect.top_left.x - rect.bottom_right.x).abs() * (rect.top_left.y - rect.bottom_right.y).abs()
}


fn main() {
    let name = String::from("Peter");
    let age = 27;

    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    let point: Point = Point {x: 10.3, y: 0.4};

    let another_point: Point = Point {x: 5.2, y: 0.2};

    println!("print coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point {x: 5.2, ..another_point};

    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}