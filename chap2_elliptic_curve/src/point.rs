pub struct Point<size> {
    pub a: size,
    pub b: size,
    pub x: size,
    pub y: size,
}

impl Point<size> {
    pub fn new(a: size, b: size, x: size, y: size) -> Option<Point<size>> {
        if y**2 == x**3 + a*x + b {
            Some(Point{a: a, b: b, x: x, y: y})
        }
        else {
            None
        }
    }
}
impl PartialEq for Point<size>{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    #[test]
    fn can_init(){
        assert!(Point{5,7,-1,-1});
    }

    #[test]
    fn is_same(){
        assert_eq!(Point{a:5, b:7, x: -1, y: -1},Point{a:5, b:7, x: -1, y: -1});
    }
}
