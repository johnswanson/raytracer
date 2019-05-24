use core::ops;
use std::num;
#[derive(Debug)]
pub struct Coord {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        is_equal(self.x, other.x)
            && is_equal(self.y, other.y)
            && is_equal(self.z, other.z)
            && is_equal(self.w, other.w)
    }
}

impl ops::Div<f64> for &Coord {
    type Output = Coord;
    fn div(self, other: f64) -> Coord {
        self * (1.0 / other)
    }
}

impl ops::Mul<&Coord> for &Coord {
    type Output = Coord;
    fn mul(self, other: &Coord) -> Coord {
        Coord::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl ops::Mul<f64> for &Coord {
    type Output = Coord;
    fn mul(self, other: f64) -> Coord {
        Coord {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl ops::Sub for &Coord {
    type Output = Coord;
    fn sub(self, other: &Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl ops::Add for &Coord {
    type Output = Coord;
    fn add(self, other: &Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Coord {
    fn point(x: f64, y: f64, z: f64) -> Coord {
        Coord { x, y, z, w: 1.0 }
    }
    fn vector(x: f64, y: f64, z: f64) -> Coord {
        Coord { x, y, z, w: 0.0 }
    }
    fn magnitude(&self) -> f64 {
        return (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt();
    }
    fn normalize(&self) -> Self {
        let mag = self.magnitude();
        self / mag
    }
    fn is_vector(&self) -> bool {
        is_equal(self.w, 0.0)
    }
    fn is_point(&self) -> bool {
        is_equal(self.w, 1.0)
    }
    fn negate(&self) -> Coord {
        &Coord::vector(0.0, 0.0, 0.0) - &self
    }
    fn dot(&self, other: &Coord) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

pub fn is_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < core::f64::EPSILON
}

#[cfg(test)]
mod tests {
    use crate::tuples::Coord;
    #[test]
    fn a_tuple_with_w_1_is_a_point() {
        let t = Coord {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert!(t.is_point());
        assert!(!t.is_vector());
    }
    #[test]
    fn a_tuple_with_w_0_is_a_vector() {
        let t = Coord {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
        assert!(!t.is_point());
        assert!(t.is_vector());
    }
    #[test]
    fn point_creates_tuples_with_w_1() {
        let p = Coord::point(4.0, -4.0, 3.0);
        assert_eq!(
            p,
            Coord {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 1.0
            }
        )
    }
    #[test]
    fn vector_creates_tuples_with_w_0() {
        let v = Coord::vector(4.0, -4.0, 3.0);
        assert_eq!(
            v,
            Coord {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 0.0
            }
        )
    }
    #[test]
    fn adding_two_tuples() {
        let a1 = Coord::vector(3.0, -2.0, 5.0);
        let a2 = Coord::point(-2.0, 3.0, 1.0);
        assert_eq!(&a1 + &a2, Coord::point(1.0, 1.0, 6.0))
    }
    #[test]
    fn subtracting_two_points() {
        let p1 = Coord::point(3.0, 2.0, 1.0);
        let p2 = Coord::point(5.0, 6.0, 7.0);
        assert_eq!(&p1 - &p2, Coord::vector(-2.0, -4.0, -6.0))
    }
    #[test]
    fn subtracting_vector_from_point() {
        let p = Coord::point(3.0, 2.0, 1.0);
        let v = Coord::vector(5.0, 6.0, 7.0);
        assert_eq!(&p - &v, Coord::point(-2.0, -4.0, -6.0))
    }
    #[test]
    fn subtracting_two_vectors() {
        let v1 = Coord::vector(3.0, 2.0, 1.0);
        let v2 = Coord::vector(5.0, 6.0, 7.0);
        assert_eq!(&v1 - &v2, Coord::vector(-2.0, -4.0, -6.0))
    }
    #[test]
    fn subtracting_a_vector_from_zero() {
        let zero = Coord::vector(0.0, 0.0, 0.0);
        let v = Coord::vector(1.0, -2.0, 3.0);
        assert_eq!(&zero - &v, Coord::vector(-1.0, 2.0, -3.0))
    }
    #[test]
    fn negating_a_tuple() {
        let v = Coord::vector(1.0, -2.0, 3.0);
        assert_eq!(v.negate(), Coord::vector(-1.0, 2.0, -3.0))
    }
    #[test]
    fn multiplying_tuple_by_scalar() {
        let a = Coord {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert_eq!(
            &a * 3.5,
            Coord {
                x: 3.5,
                y: -7.0,
                z: 10.5,
                w: -14.0
            }
        )
    }
    #[test]
    fn multiplying_tuple_by_fraction() {
        let a = Coord {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert_eq!(
            &a * 0.5,
            Coord {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            }
        )
    }
    #[test]
    fn dividing_tuple_by_scalar() {
        let a = Coord {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert_eq!(
            &a / 2.0,
            Coord {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            }
        )
    }
    #[test]
    fn computing_magnitude_of_vectors() {
        let v = Coord::vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
        let v = Coord::vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
        let v = Coord::vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
        let v = Coord::vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), (14_f64).sqrt());
        let v = Coord::vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), (14_f64).sqrt());
    }
    #[test]
    fn normalizing_vectors() {
        let v = Coord::vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Coord::vector(1.0, 0.0, 0.0));
        let v = Coord::vector(1.0, 2.0, 3.0);
        assert_eq!(
            v.normalize(),
            Coord::vector(
                1_f64 / (14_f64).sqrt(),
                2_f64 / (14_f64).sqrt(),
                3_f64 / (14_f64).sqrt(),
            )
        );
        assert_eq!(v.normalize().magnitude(), 1_f64);
    }
    #[test]
    fn dot_product_of_two_tuples() {
        let a = Coord::vector(1.0, 2.0, 3.0);
        let b = Coord::vector(2.0, 3.0, 4.0);
        assert_eq!(a.dot(&b), 20_f64);
    }
    #[test]
    fn cross_product_of_two_vectors() {
        let a = Coord::vector(1.0, 2.0, 3.0);
        let b = Coord::vector(2.0, 3.0, 4.0);
        assert_eq!(&a * &b, Coord::vector(-1.0, 2.0, -1.0));
        assert_eq!(&b * &a, Coord::vector(1.0, -2.0, 1.0));
    }
}
