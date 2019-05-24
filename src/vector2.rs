use std::fmt;
use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ops::Add for &Vector2 {
    type Output = Vector2;

    fn add(self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for &Vector2 {
    type Output = Vector2;

    fn sub(self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Vector2 {
    pub fn origin() -> Vector2 {
        Vector2 { x: 0.0, y: 0.0 }
    }

    pub fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn is_zero(&self) -> bool {
        self.norm() == 0.0
    }

    pub fn multi(&self, c: f64) -> Vector2 {
        Vector2 {
            x: self.x * c,
            y: self.y * c,
        }
    }

    pub fn divide(&self, c: f64) -> Vector2 {
        Vector2 {
            x: self.x / c,
            y: self.y / c,
        }
    }

    pub fn unit(&self) -> Result<Vector2, Vector2> {
        let n = self.norm();
        if n == 0.0 {
            Err(Vector2::origin())
        } else {
            Ok(self.divide(n))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let v1 = Vector2 { x: 3.0, y: 4.0 };
        let v2 = Vector2 { x: 10.0, y: 20.0 };
        assert_eq!(Vector2 { x: 13.0, y: 24.0 }, &v1 + &v2);
    }

    #[test]
    fn sub() {
        let v1 = Vector2 { x: 3.0, y: 4.0 };
        let v2 = Vector2 { x: 10.0, y: 20.0 };
        assert_eq!(Vector2 { x: -7.0, y: -16.0 }, &v1 - &v2);
    }

    #[test]
    fn origin() {
        let v1 = Vector2::origin();
        assert_eq!(Vector2 { x: 0.0, y: 0.0 }, v1);
    }

    #[test]
    fn norm() {
        let v1 = Vector2 { x: 3.0, y: 4.0 };
        assert_eq!(5.0, v1.norm());
    }

    #[test]
    fn is_zero() {
        let v1 = Vector2 { x: 3.0, y: 4.0 };
        let v2 = Vector2 { x: 0.0, y: 0.0 };
        assert_eq!(false, v1.is_zero());
        assert_eq!(true, v2.is_zero());
    }

    #[test]
    fn multi() {
        let v1 = Vector2 { x: 3.0, y: 4.0 };
        assert_eq!(Vector2 { x: 6.0, y: 8.0 }, v1.multi(2.0));
    }

    #[test]
    fn divide() {
        let v1 = Vector2 { x: 3.0, y: 4.0 };
        assert_eq!(Vector2 { x: 1.5, y: 2.0 }, v1.divide(2.0));
    }

    #[test]
    fn unit() {
        let v1 = Vector2 { x: 3.0, y: 4.0 };
        match v1.unit() {
            Ok(v) => assert_eq!(Vector2 { x: 0.6, y: 0.8 }, v),
            Err(v) => panic!(v),
        }
        let v2 = Vector2::origin();
        match v2.unit() {
            Ok(v) => panic!(v),
            Err(v) => assert_eq!(Vector2 { x: 0.0, y: 0.0 }, v),
        }
    }
}
