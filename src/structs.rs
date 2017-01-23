fn main() {
    struct Nil {};
    struct AlsoNil;

    struct Color(i32, i32, i32);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);


    struct Point {
        x: f32,
        y: f32,
    }

    let mut point: Point = Point{x: 0.7, y: 0.9};

    println!("Black: ({:?},{:?},{:?})", black.0, black.1, black.2);
    println!("White: ({:?},{:?},{:?})", white.0, white.1, white.2);
    println!("({:?},{:?})", point.x, point.y);

}
