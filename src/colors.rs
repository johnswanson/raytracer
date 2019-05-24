use crate::tuples::is_equal;
use core::ops;
use std::num;
#[derive(Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    fn color(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        is_equal(self.red, other.red)
            && is_equal(self.green, other.green)
            && is_equal(self.blue, other.blue)
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
    #[test]
    fn colors_are_red_green_blue_tuples() {
        let c = Color {
            red: -0.5,
            green: 0.4,
            blue: 1.7,
        };
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }
    #[test]
    fn adding_colors() {
        let c1 = Color::color(0.9, 0.6, 0.75);
        let c2 = Color::color(0.7, 0.1, 0.25);
        assert_eq!(&c1 + &c2, Color::color(1.6, 0.7, 1.0))
    }
}
