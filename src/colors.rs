use crate::tuples::ApproxEq;
use core::ops;

const MIN: f64 = 0.0;
const MAX: f64 = 1.0;

#[derive(Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

fn clamp(v: f64) -> f64 {
    if v < MIN {
        return MIN;
    }
    if v > MAX {
        return MAX;
    }
    return v;
}

impl Color {
    fn color(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }
    pub fn clamped(&self) -> Color {
        Color {
            red: clamp(self.red),
            green: clamp(self.green),
            blue: clamp(self.blue),
        }
    }
}

impl ApproxEq<&Color> for &Color {
    fn approx_eq(self, other: &Color) -> bool {
        (self.red.approx_eq(other.red))
            && (self.green.approx_eq(other.green))
            && (self.blue.approx_eq(other.blue))
    }
}

impl ops::Add for &Color {
    type Output = Color;
    fn add(self, other: &Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl ops::Sub for &Color {
    type Output = Color;
    fn sub(self, other: &Color) -> Color {
        self + &(other * -1.0)
    }
}

impl ops::Mul<f64> for &Color {
    type Output = Color;
    fn mul(self, other: f64) -> Color {
        Color {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other,
        }
    }
}

impl ops::Mul<&Color> for &Color {
    type Output = Color;
    fn mul(self, other: &Color) -> Color {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::colors::Color;
    use crate::tuples::ApproxEq;
    #[test]
    fn colors_are_red_green_blue_tuples() {
        let c = Color {
            red: -0.5,
            green: 0.4,
            blue: 1.7,
        };
        assert!(c.red.approx_eq(-0.5));
        assert!(c.green.approx_eq(0.4));
        assert!(c.blue.approx_eq(1.7));
    }
    #[test]
    fn adding_colors() {
        let c1 = Color::color(0.9, 0.6, 0.75);
        let c2 = Color::color(0.7, 0.1, 0.25);
        assert!((&c1 + &c2).approx_eq(&Color::color(1.6, 0.7, 1.0)))
    }
}
