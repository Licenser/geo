trait Geometry {
    fn new() -> Self;
    fn draw();
}

struct Circle {}
impl Geometry for Circle {
    fn draw() {
        println!("()")
    }
    fn new() -> Self {
        Circle {}
    }
}
struct Square {}
impl Geometry for Square {
    fn draw() {
        println!("[]")
    }
    fn new() -> Self {
        Square {}
    }
}

fn new_geo(shape: &str) -> Box<Geometry> {
    match shape {
        "square" => Square::new(),
        "circle" => Circle::new(),
        other => panic!("Bad shape: {}", other),
    }
}

fn main() {
    println!("Hello, world!");
}
