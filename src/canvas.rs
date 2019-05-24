use crate::colors;
use crate::colors::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        let mut pixels = Vec::new();
        for i in 0..height {
            let mut row = Vec::new();
            for i in 0..width {
                let pixel = Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                };
                row.push(pixel);
            }
            pixels.push(row);
        }
        Canvas {
            width,
            height,
            pixels,
        }
    }
    fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[x][y] = color
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::colors::Color;
    #[test]
    fn creating_canvas() {
        let canvas = Canvas::new(10 as usize, 20 as usize);
        assert_eq!(10 as usize, canvas.width);
        assert_eq!(20 as usize, canvas.height);
        assert_eq!(Vec::len(&canvas.pixels), canvas.height);
    }
    fn writing_pixel() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
        };
        canvas.write_pixel(2, 3, red);
        assert_eq!(
            Color {
                red: 1.0,
                green: 0.0,
                blue: 0.0,
            },
            canvas.pixels[2][3]
        );
    }
}
