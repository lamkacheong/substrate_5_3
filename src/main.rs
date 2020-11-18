trait HasArea {
    fn area(&self) -> f32;
}

struct Rectangle{
    length: f32,
    weight: f32,
}

impl HasArea for Rectangle{
    fn area(&self) -> f32 {
        self.weight * self.length
    }
}
struct Circle{
    radius: f32,
}

impl HasArea for Circle{
    fn area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}

struct Square{
    side: f32,
}

impl HasArea for Square{
    fn area(&self) -> f32 {
        self.side * self.side
    }
}

fn main() {
    let circle = Circle{
        radius: 2.0,
    };
    let square = Square{
        side: 2.0,
    };
    let rectangle = Rectangle{
        weight: 4.0,
        length: 5.0,
    };
    println!("{}", circle.area());
    println!("{}", square.area());
    println!("{}", rectangle.area());
}