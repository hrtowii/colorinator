// https://github.com/nix-community/stylix/blob/master/palette-generator/Data/Colour.hs

#[derive(Debug, Clone, Copy)]
pub enum Color {
    RGB(RGB),
    LAB(LAB),
}

#[derive(Debug, Clone, Copy)]
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Clone, Copy)]
struct LAB {
    lightness: f64,
    channel_a: f64,
    channel_b: f64,
}

impl From<RGB> for Color {
    fn from(rgb: RGB) -> Self {
        Self::RGB(rgb)
    }
}

impl From<LAB> for Color {
    fn from(lab: LAB) -> Self {
        Self::LAB(lab)
    }
}

impl From<LAB> for RGB {
    fn from(lab: LAB) -> RGB {
        todo!()
    }
}

impl From<RGB> for LAB {
    fn from(rgb: RGB) -> LAB {
        // 1: pivot

        todo!()
    }
}

impl Color {
    pub fn to_hex(&self) -> String {
        let rgb = match self {
            Color::RGB(rgb) => *rgb,
            Color::LAB(lab) => RGB::from(*lab),
        };
        format!("#{:02X}{:02X}{:02X}", rgb.red, rgb.green, rgb.blue)
    }
}

pub struct Base16 {
    colors: [Color; 16],
}
