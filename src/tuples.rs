use core::ops;
#[derive(Debug)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
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
    pub fn point(x: f64, y: f64, z: f64) -> Coord {
        Coord { x, y, z, w: 1.0 }
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Coord {
        Coord { x, y, z, w: 0.0 }
    }
    fn magnitude(&self) -> f64 {
        return (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt();
    }
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        self / mag
    }
    fn is_vector(&self) -> bool {
        self.w.approx_eq(0.)
    }
    fn is_point(&self) -> bool {
        self.w.approx_eq(1.)
    }
    fn negate(&self) -> Coord {
        &Coord::vector(0.0, 0.0, 0.0) - &self
    }
    fn dot(&self, other: &Coord) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

pub trait ApproxEq<T> {
    fn approx_eq(self, other: T) -> bool;
}

impl ApproxEq<f64> for f64 {
    fn approx_eq(self, other: f64) -> bool {
        f64::abs(self - other) < core::f64::EPSILON
    }
}

impl ApproxEq<&Coord> for &Coord {
    fn approx_eq(self, other: &Coord) -> bool {
        self.x.approx_eq(other.x) &&
            self.y.approx_eq(other.y) &&
            self.z.approx_eq(other.z) &&
            self.w.approx_eq(other.w)
    }
}

#[cfg(test)]
mod tests {
    use crate::tuples::Coord;
    use crate::tuples::ApproxEq;
    #[test]
    fn approx_eq_works() {
        let a = 10000.2;
        let b = 5000. + 5000. + 0.2;
        assert!(a.approx_eq(b));
    }
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
        assert!(
            p.approx_eq(
                &Coord {
                    x: 4.0,
                    y: -4.0,
                    z: 3.0,
                    w: 1.0
                }
            )
        )
    }
    #[test]
    fn vector_creates_tuples_with_w_0() {
        let v = Coord::vector(4.0, -4.0, 3.0);
        assert!(
            v.approx_eq(
                &Coord {
                    x: 4.0,
                    y: -4.0,
                    z: 3.0,
                    w: 0.0
                }
            )
        )
    }
    #[test]
    fn adding_two_tuples() {
        let a1 = Coord::vector(3.0, -2.0, 5.0);
        let a2 = Coord::point(-2.0, 3.0, 1.0);
        assert!((&a1 + &a2).approx_eq(&Coord::point(1., 1., 6.)))
    }
    #[test]
    fn subtracting_two_points() {
        let p1 = Coord::point(3.0, 2.0, 1.0);
        let p2 = Coord::point(5.0, 6.0, 7.0);
        assert!((&p1 - &p2).approx_eq(&Coord::vector(-2., -4., -6.)))
    }
    #[test]
    fn subtracting_vector_from_point() {
        let p = Coord::point(3.0, 2.0, 1.0);
        let v = Coord::vector(5.0, 6.0, 7.0);
        assert!((&p - &v).approx_eq(&Coord::point(-2.0, -4.0, -6.0)))
    }
    #[test]
    fn subtracting_two_vectors() {
        let v1 = Coord::vector(3.0, 2.0, 1.0);
        let v2 = Coord::vector(5.0, 6.0, 7.0);
        assert!((&v1 - &v2).approx_eq(&Coord::vector(-2.0, -4.0, -6.0)))
    }
    #[test]
    fn subtracting_a_vector_from_zero() {
        let zero = Coord::vector(0.0, 0.0, 0.0);
        let v = Coord::vector(1.0, -2.0, 3.0);
        assert!((&zero - &v).approx_eq(&Coord::vector(-1.0, 2.0, -3.0)))
    }
    #[test]
    fn negating_a_tuple() {
        let v = Coord::vector(1.0, -2.0, 3.0);
        assert!((v.negate()).approx_eq(&Coord::vector(-1.0, 2.0, -3.0)))
    }
    #[test]
    fn multiplying_tuple_by_scalar() {
        let a = Coord {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert!(
            (&a * 3.5).approx_eq(
                &Coord {
                    x: 3.5,
                    y: -7.0,
                    z: 10.5,
                    w: -14.0
                }
            )
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
        assert!(
            (&a * 0.5).approx_eq(&Coord {x: 0.5, y: -1.0, z: 1.5, w: -2.0 })
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
        assert!(
            (&a / 2.0).approx_eq(
                &Coord {
                    x: 0.5,
                    y: -1.0,
                    z: 1.5,
                    w: -2.0
                }
            )
        )
    }
    #[test]
    fn computing_magnitude_of_vectors() {
        let v = Coord::vector(1.0, 0.0, 0.0);
        assert!(v.magnitude().approx_eq(1.0));
        let v = Coord::vector(0.0, 1.0, 0.0);
        assert!(v.magnitude().approx_eq(1.0));
        let v = Coord::vector(0.0, 0.0, 1.0);
        assert!(v.magnitude().approx_eq(1.0));
        let v = Coord::vector(1.0, 2.0, 3.0);
        assert!(v.magnitude().approx_eq((14_f64).sqrt()));
        let v = Coord::vector(-1.0, -2.0, -3.0);
        assert!(v.magnitude().approx_eq((14_f64).sqrt()));
    }
    #[test]
    fn normalizing_vectors() {
        let v = Coord::vector(4.0, 0.0, 0.0);
        assert!(v.normalize().approx_eq(&Coord::vector(1.0, 0.0, 0.0)));
        let v = Coord::vector(1.0, 2.0, 3.0);
        assert!(
            v.normalize().approx_eq(
                &Coord::vector(
                    1_f64 / (14_f64).sqrt(),
                    2_f64 / (14_f64).sqrt(),
                    3_f64 / (14_f64).sqrt(),
                )
            )
        );
        assert!(v.normalize().magnitude().approx_eq(1_f64));
    }
    #[test]
    fn dot_product_of_two_tuples() {
        let a = Coord::vector(1.0, 2.0, 3.0);
        let b = Coord::vector(2.0, 3.0, 4.0);
        assert!(a.dot(&b).approx_eq(20_f64));
    }
    #[test]
    fn cross_product_of_two_vectors() {
        let a = Coord::vector(1.0, 2.0, 3.0);
        let b = Coord::vector(2.0, 3.0, 4.0);
        assert!((&a * &b).approx_eq(&Coord::vector(-1.0, 2.0, -1.0)));
        assert!((&b * &a).approx_eq(&Coord::vector(1.0, -2.0, 1.0)));
    }
}
