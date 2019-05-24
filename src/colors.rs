use core::ops;
use std::num;
#[derive(Debug)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
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
}
