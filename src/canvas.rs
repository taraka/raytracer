use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::black(); width * height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let i = self.index(x, y);
        if let Some(c) = self.pixels.get_mut(i) {
            *c = color;
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels.get(self.index(x, y)).unwrap().clone()
    }

    #[inline]
    fn index(&self, x: usize, y: usize) -> usize {
        (self.width * y) + x
    }

    pub fn to_ppm(&self) -> String {
        let mut data = format!("P3\n{} {}\n225", self.width, self.height).to_string();
        let mut curr_line_len = 0;
        for i in 0..self.pixels.len() {
            if i % self.width == 0 {
                data.push('\n');
                curr_line_len = 0;
            }

            let mut wp = |v: f64| {
                let s = &format!("{} ", (v * 255.0).round() as u8);
                curr_line_len += s.len();
                if curr_line_len > 70 {
                    data.push('\n');
                    curr_line_len = 0;
                }
                data.push_str(s);
            };

            let c = self.pixels[i];
            wp(c.red);
            wp(c.green);
            wp(c.blue);
        }
        data.push('\n');
        data
    }

    pub fn fill(&mut self, c: Color) {
        for p in self.pixels.iter_mut() {
            *p = c;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::color::Color;

    #[test]
    fn create_canvas() {
        let canvas = Canvas::new(10, 20);

        assert_eq!(10, canvas.width);
        assert_eq!(20, canvas.height);

        assert_eq!(200, canvas.pixels.len());
        assert!(canvas.pixels.iter().all(|&c| c.is_black()));
    }

    #[test]
    fn write_pixel_to_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        canvas.write_pixel(2, 3, red);

        assert_eq!(red, canvas.pixel_at(2, 3));
    }

    #[test]
    fn canvas_ppm_header() {
        let canvas = Canvas::new(5, 3);

        assert!(canvas.to_ppm().starts_with("P3\n5 3\n225\n"));
    }

    #[test]
    fn canvas_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);

        canvas.write_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        canvas.write_pixel(2, 1, Color::new(0.0, 0.5, 0.0));
        canvas.write_pixel(4, 2, Color::new(-0.5, 0.0, 1.0));
        assert_eq!(
            r#"P3
5 3
225
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 
"#,
            canvas.to_ppm()
        );
    }

    #[test]
    fn canvas_ppm_pixel_data_line_length() {
        let mut canvas = Canvas::new(10, 2);
        canvas.fill(Color::new(1.0, 0.8, 0.6));

        assert_eq!(
            r#"P3
10 2
225
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 
153 255 204 153 255 204 153 255 204 153 255 204 153 
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 
153 255 204 153 255 204 153 255 204 153 255 204 153 
"#,
            canvas.to_ppm()
        );
    }

    #[test]
    fn canvas_ppm_pixel_data_newline_end() {
        let canvas = Canvas::new(5, 3);
        assert!(canvas.to_ppm().ends_with("\n"));
    }
}
