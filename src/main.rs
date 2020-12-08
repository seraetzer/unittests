struct Circle {
    radius: f32,
    area: f32,
}

impl Circle {
    fn new(radius: f32) -> Circle {
        Circle { radius, area: radius*radius*3.1415 }
    }
    
    fn is_larger(&self, other: &Circle) -> bool {
        self.radius > other.radius && self.area > other.area
    }
}

fn main() {
    println!("A Test Module")
}

#[cfg(test)]
mod tests {
    use crate::Circle;
    #[test]
    fn testit() { // Test if one circle ist larger than another

        let larger = Circle::new(3.0);
        let smaller = Circle::new(2.0);

        assert!(larger.is_larger(&smaller))
    }
}
