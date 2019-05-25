use std::f64;
use std::fmt;
use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl ops::Add for &Vector2 {
    type Output = Vector2;

    /// Add tow vectors
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_polygon_parser::vector2::Vector2;
    ///
    /// let v1 = Vector2::new(1.0, 2.0);
    /// let v2 = Vector2::new(3.0, 4.0);
    /// assert_eq!(&v1 + &v2, Vector2::new(4.0, 6.0));
    /// ```
    fn add(self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for &Vector2 {
    type Output = Vector2;

    /// Sub tow vectors
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_polygon_parser::vector2::Vector2;
    ///
    /// let v1 = Vector2::new(1.0, 2.0);
    /// let v2 = Vector2::new(3.0, 4.0);
    /// assert_eq!(&v1 - &v2, Vector2::new(-2.0, -2.0));
    /// ```
    fn sub(self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Vector2 {
    /// Returns a Vector2
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_polygon_parser::vector2::Vector2;
    ///
    /// let v1 = Vector2::new(1.0, 2.0);
    /// assert_eq!(v1, Vector2 { x: 1.0, y: 2.0 });
    /// ```
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    /// Returns a origin
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_polygon_parser::vector2::Vector2;
    ///
    /// let v1 = Vector2::origin();
    /// assert_eq!(v1, Vector2::new(0.0, 0.0));
    /// ```
    pub fn origin() -> Vector2 {
        Vector2 { x: 0.0, y: 0.0 }
    }

    /// Returns a norm
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_polygon_parser::vector2::Vector2;
    ///
    /// let v1 = Vector2::new(3.0, 4.0);
    /// assert_eq!(v1.norm(), 5.0);
    /// ```
    pub fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Returns true if a vector is zero
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_polygon_parser::vector2::Vector2;
    ///
    /// let v = Vector2::new(0.0, 0.0);
    /// assert!(v.is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        self.norm() == 0.0
    }

    /// Returns a multiplied vector
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_polygon_parser::vector2::Vector2;
    ///
    /// let v = Vector2::new(3.0, 4.0);
    /// assert_eq!(v.multi(2.0), Vector2::new(6.0,  8.0));
    /// ```
    pub fn multi(&self, c: f64) -> Vector2 {
        Vector2 {
            x: self.x * c,
            y: self.y * c,
        }
    }

    /// Returns a divided vector
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_polygon_parser::vector2::Vector2;
    ///
    /// let v = Vector2::new(3.0, 4.0);
    /// assert_eq!(v.divide(2.0), Vector2::new(1.5,  2.0));
    /// ```
    pub fn divide(&self, c: f64) -> Vector2 {
        Vector2 {
            x: self.x / c,
            y: self.y / c,
        }
    }

    /// Returns a unit vector
    ///
    /// # Examples
    ///
    /// A nonzero vecotr returns Ok
    ///
    /// ```
    /// use svg_polygon_parser::vector2::Vector2;
    ///
    /// let v = Vector2::new(3.0, 4.0);
    /// assert_eq!(v.unit(), Ok(Vector2::new(0.6, 0.8)));
    /// ```
    ///
    /// # Failures
    ///
    /// A zero vecotr returns Err
    ///
    /// ```
    /// use svg_polygon_parser::vector2::Vector2;
    ///
    /// let v = Vector2::new(0.0, 0.0);
    /// assert_eq!(v.unit(), Err(Vector2::new(0.0, 0.0)));
    /// ```
    pub fn unit(&self) -> Result<Vector2, Vector2> {
        let n = self.norm();
        if n == 0.0 {
            Err(Vector2::origin())
        } else {
            Ok(self.divide(n))
        }
    }

    /// Returns self.y.atan2(slef.x)
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_polygon_parser::vector2::Vector2;
    ///
    /// let v1 = Vector2::new(1.0, 1.0);
    /// assert_eq!(v1.radian(), std::f64::consts::FRAC_PI_4);
    ///
    /// let v2 = Vector2::new(1.0, -1.0);
    /// assert_eq!(v2.radian(), -std::f64::consts::FRAC_PI_4);
    /// ```
    pub fn radian(&self) -> f64 {
        self.y.atan2(self.x)
    }
}
