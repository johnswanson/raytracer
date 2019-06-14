use crate::colors::Color;

const MAGIC_NUMBER: &str = "P3";
const MAX_COLOR_VALUE: u64 = 255;

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut pixels = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
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
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.pixels[y][x] = color
    }
}

fn color_to_string(color: &Color) -> String {
    let clamped = color.clamped();
    format!(
        "{red} {green} {blue}",
        red = (255 as f64 * clamped.red).round() as u8,
        green = (255 as f64 * clamped.green).round() as u8,
        blue = (255 as f64 * clamped.blue).round() as u8,
    )
}

fn split_long_lines(s: String) -> String {
    let mut new_s = String::new();
    let words: Vec<&str> = s.split(" ").collect();
    let mut current_length = 0;
    for (i, &word) in words.iter().enumerate() {
        new_s += word;
        current_length += word.chars().count();
        let next_length = if i < words.iter().count() - 1 {
            current_length + words[i + 1].chars().count() + 1
        } else {
            current_length
        };
        if next_length > 70 {
            current_length = 0;
            new_s += "\n";
        } else if i < words.iter().count() - 1 {
            current_length += 1;
            new_s += " ";
        }
    }
    new_s
}

fn row_to_string(colors: &Vec<Color>) -> String {
    return colors
        .iter()
        .map(|c| color_to_string(c))
        .collect::<Vec<String>>()
        .join(" ");
}

pub fn canvas_to_ppm(canvas: &Canvas) -> String {
    format!(
        "{magic_number}\n{width} {height}\n{max_color_value}\n{pixels}\n",
        magic_number = MAGIC_NUMBER,
        width = canvas.width,
        height = canvas.height,
        max_color_value = MAX_COLOR_VALUE,
        pixels = canvas
            .pixels
            .iter()
            .map(|row| row_to_string(row))
            .map(|s| split_long_lines(s))
            .collect::<Vec<String>>()
            .join("\n"),
    )
}

#[cfg(test)]
mod tests {
    use crate::canvas;
    use crate::canvas::Canvas;
    use crate::colors::Color;
    #[test]
    fn creating_canvas() {
        let canvas = Canvas::new(10 as usize, 20 as usize);
        assert_eq!(10 as usize, canvas.width);
        assert_eq!(20 as usize, canvas.height);
        assert_eq!(Vec::len(&canvas.pixels), canvas.height);
    }
    #[test]
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
            canvas.pixels[3][2]
        );
    }
    #[test]
    fn constructing_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = canvas::canvas_to_ppm(&c);
        let lines: Vec<&str> = ppm.lines().collect();
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "255");
    }
    #[test]
    fn constructing_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color {
            red: 1.5,
            green: 0.0,
            blue: 0.0,
        };
        let c2 = Color {
            red: 0.0,
            green: 0.5,
            blue: 0.0,
        };
        let c3 = Color {
            red: -0.5,
            green: 0.0,
            blue: 1.0,
        };
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let ppm = canvas::canvas_to_ppm(&c);
        let ppm_lines: Vec<&str> = ppm.lines().collect();
        assert_eq!(
            ppm_lines[3..],
            [
                "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
            ]
        )
    }
    #[test]
    fn splitting_long_lines() {
        let mut c = Canvas::new(10, 2);
        for y in 0..c.height {
            for x in 0..c.width {
                let color = Color {
                    red: 1.0,
                    green: 0.8,
                    blue: 0.6,
                };
                c.write_pixel(x, y, color)
            }
        }
        let ppm = canvas::canvas_to_ppm(&c);
        let ppm_lines: Vec<&str> = ppm.lines().collect();
        assert_eq!(
            ppm_lines[3..],
            [
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153",
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153",
            ]
        )
    }
}
