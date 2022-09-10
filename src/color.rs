use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn is_black(&self) -> bool {
        self.red == 0.0 && self.green == 0.0 && self.blue == 0.0
    }

    pub fn is_white(&self) -> bool {
        self.red == 1.0 && self.green == 1.0 && self.blue == 1.0
    }
}

// Really not sure why I've had to go with 10x EPSILON here but I was getting issues when adding 0.1 to 0.6 and comparing to 0.7
impl PartialEq<Color> for Color {
    fn eq(&self, rhs: &Color) -> bool {
        println!("{}, {}", (self.green - rhs.green).abs(), f64::EPSILON);
        (self.red - rhs.red).abs() < f64::EPSILON * 10.0
            && (self.green - rhs.green).abs() < f64::EPSILON * 10.0
            && (self.blue - rhs.blue).abs() < f64::EPSILON * 10.0
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self {
        Self::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self {
        Self::new(
            self.red - rhs.red,
            self.green - rhs.green,
            self.blue - rhs.blue,
        )
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self {
        Self::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self {
        Self::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn colors_are_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn add_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtract_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiply_colors_by_scalar() {
        assert_eq!(Color::new(0.2, 0.3, 0.4) * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiply_colors_by_color() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }

    #[test]
    fn black_is_black() {
        let c = Color::black();
        assert!(c.is_black());
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn white_is_white() {
        let c = Color::white();
        assert!(c.is_white());
        assert_eq!(c, Color::new(1.0, 1.0, 1.0));
    }
}
