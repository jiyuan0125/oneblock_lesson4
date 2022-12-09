pub trait Shape {
    fn get_area(&self) -> Option<f64>;
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> anyhow::Result<Rectangle> {
        if width < 0f64 || height < 0f64 {
            return Err(anyhow::anyhow!("Width and height must be positive"));
        }
        Ok(Rectangle { width, height })
    }
}

impl Shape for Rectangle {
    fn get_area(&self) -> Option<f64> {
        // check for overflow
        if f64::MAX / self.width < self.height {
            return None;
        }
        Some(self.width * self.height)
    }
}

pub struct Square {
    width: f64,
}

impl Square {
    pub fn new(width: f64) -> anyhow::Result<Square> {
        if width < 0f64 {
            return Err(anyhow::anyhow!("Width must be positive"));
        }
        Ok(Square { width })
    }
}

impl Shape for Square {
    fn get_area(&self) -> Option<f64> {
        // check for overflow
        if f64::MAX / self.width < self.width {
            return None;
        }
        Some(self.width * self.width)
    }
}

pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> anyhow::Result<Circle> {
        if radius < 0f64 {
            return Err(anyhow::anyhow!("Radius must be positive"));
        }
        Ok(Circle { radius })
    }
}

impl Shape for Circle {
    fn get_area(&self) -> Option<f64> {
        use std::f64::consts::PI;
        // check for overflow
        if f64::MAX / self.radius < self.radius || f64::MAX / (self.radius * self.radius) < PI {
            return None;
        }
        Some(self.radius * self.radius * PI)
    }
}

pub fn get_area<T: Shape>(shape: &T) -> Option<f64> {
    shape.get_area()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_area() {

        let rectangle = Rectangle::new(10f64, 20f64).unwrap();
        assert_eq!(get_area(&rectangle), Some(200f64));

        let rectangle = Rectangle::new(f64::MAX, f64::MAX).unwrap();
        assert_eq!(get_area(&rectangle), None);

        let square = Square::new(f64::MAX).unwrap();
        assert_eq!(get_area(&square), None);

        let square = Square::new(10f64).unwrap();
        assert_eq!(get_area(&square), Some(100f64));

        let circle = Circle::new(10f64).unwrap();
        assert_eq!(get_area(&circle), Some(314.1592653589793f64));

        let circle = Circle::new(f64::MAX).unwrap();
        assert_eq!(get_area(&circle), None);
    }
}
