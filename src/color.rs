use na::Vector4;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}


impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::rgba(r, g, b, 255)
    }

    pub fn black() -> Self {
        Color::rgb(0, 0, 0)
    }

    pub fn white() -> Self {
        Color::rgb(255, 255, 255)
    }

    pub fn gray() -> Self {
        Color::rgb(128, 128, 128)
    }

    pub fn red() -> Self {
        Color::rgb(255, 0, 0)
    }

    pub fn green() -> Self {
        Color::rgb(0, 255, 0)
    }

    pub fn blue() -> Self {
        Color::rgb(0, 0, 255)
    }
}


impl Into<Vector4<f32>> for Color {
    fn into(self) -> Vector4<f32> {
        Vector4::new(
            (self.r as f32) / 255.0,
            (self.g as f32) / 255.0,
            (self.b as f32) / 255.0,
            (self.a as f32) / 255.0,
        )
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [
            (self.r as f32) / 255.0,
            (self.g as f32) / 255.0,
            (self.b as f32) / 255.0,
            (self.a as f32) / 255.0,
        ]
    }
}
