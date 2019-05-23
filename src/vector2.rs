use std::fmt;
use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ops::Add for &Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for &Vector {
    type Output = Vector;

    fn sub(self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Vector {
    fn norm(&self) -> f64 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }

    fn is_zero(&self) -> bool {
        return self.norm() == 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let v1 = Vector { x: 3.0, y: 4.0 };
        let v2 = Vector { x: 10.0, y: 20.0 };
        assert_eq!(Vector { x: 13.0, y: 24.0 }, &v1 + &v2);
    }

    #[test]
    fn sub() {
        let v1 = Vector { x: 3.0, y: 4.0 };
        let v2 = Vector { x: 10.0, y: 20.0 };
        assert_eq!(Vector { x: -7.0, y: -16.0 }, &v1 - &v2);
    }

    #[test]
    fn norm() {
        let v1 = Vector { x: 3.0, y: 4.0 };
        assert_eq!(5.0, v1.norm());
    }

    #[test]
    fn is_zero() {
        let v1 = Vector { x: 3.0, y: 4.0 };
        let v2 = Vector { x: 0.0, y: 0.0 };
        assert_eq!(false, v1.is_zero());
        assert_eq!(true, v2.is_zero());
    }
}
