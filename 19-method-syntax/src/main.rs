struct Circle {
    x: f64,
    y: f64,
    radius: f64
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius
        }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

fn main() {
    // impl keyword

    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };

    println!("{}", c.area());

    let c2 = Circle::new(0.0, 0.0, 3.0);
    println!("Constructed circle at ({},{}) with area: {}", c2.x, c2.y, c2.area());
}