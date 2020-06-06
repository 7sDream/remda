use std::borrow::Cow;

macro_rules! check0to1 {
    ($r: ident, $g: ident, $b: ident) => {
        assert!(0.0 <= $r && $r <= 1.0);
        assert!(0.0 <= $g && $g <= 1.0);
        assert!(0.0 <= $b && $b <= 1.0);
    };
}

#[derive(Debug, Clone, Default)]
pub struct RGBFloat {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl RGBFloat {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        check0to1!(r, g, b);
        Self { r, g, b }
    }
}

impl<'a> Into<RGBInt> for &'a RGBFloat {
    fn into(self) -> RGBInt {
        RGBInt::new((self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8)
    }
}

#[derive(Debug, Clone, Default)]
pub struct RGBInt {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGBInt {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[derive(Debug, Clone)]
pub enum Color {
    RGBF(RGBFloat),
    RGBI(RGBInt),
}

impl Default for Color {
    fn default() -> Self {
        Self::RGBF(RGBFloat::default())
    }
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self::RGBI(RGBInt::new(r, g, b))
    }

    pub fn newf(r: f32, g: f32, b: f32) -> Self {
        Self::RGBF(RGBFloat::new(r, g, b))
    }

    pub fn i(&self) -> Cow<'_, RGBInt> {
        match self {
            Color::RGBF(c) => Cow::Owned(c.into()),
            Color::RGBI(c) => Cow::Borrowed(c),
        }
    }
}

pub static BLACK: Color = Color::new(0, 0, 0);
pub static WHITE: Color = Color::new(255, 255, 255);
pub static RED: Color = Color::new(255, 0, 0);
pub static GREEN: Color = Color::new(0, 255, 0);
pub static BLUE: Color = Color::new(0, 0, 255);
