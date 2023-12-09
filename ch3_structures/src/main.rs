#[allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: Point { x: right_edge, y: bottom_edge },
    } = rect;

    (left_edge - right_edge).abs() * (top_edge - bottom_edge).abs()
}

fn square(point: Point, side: f32) -> Rectangle {
    let Point { x, y } = point;

    Rectangle {
        top_left: Point { x, y },
        bottom_right: Point { x: x + side, y: y + side }
    }
}


fn main() {
    let name = String::from("Peter");

    let age = 27;
    let peter = Person {name, age};

    println!("{:?}", peter);

    let point: Point = Point {x: 10.3, y: 0.4};

    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right
    };

    let pair = Pair(1, 0.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
