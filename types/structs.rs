
// three kinds of structs: 
// unit structs, which are fieldless (useful for generics)
struct Nil;
// tuple structs, which are basically named tuples
struct Pair(i32, f32);
// classic C structs
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {

    let point: Point = Point {x: 0.3, y: 0.4 };

    let Point {x: my_x, y: my_y } = point;
    // WHOA we just named my_x and my_y, that's so rad!

    let _nil = Nil;

    let pair = Pair(1, 0.1);
    let pair(integer, decimal) = pair;

    // we're instantiating this
    let _rectangle = Rectangle {
        p1: Point {x: my_y, y: my_x },
        p2: point
    }
}

