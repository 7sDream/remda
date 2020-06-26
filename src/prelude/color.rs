use crate::prelude::Point3;
use {
    super::clamp,
    crate::texture::Texture,
    std::{borrow::Cow, ops::Mul},
};

macro_rules! check0to1 {
    ($r: ident, $g: ident, $b: ident) => {
        debug_assert!(0.0 <= $r && $r <= 1.0, "r = {}", $r);
        debug_assert!(0.0 <= $g && $g <= 1.0, "g = {}", $g);
        debug_assert!(0.0 <= $b && $b <= 1.0, "b = {}", $b);
    };
}

#[derive(Debug, Clone, Default)]
pub struct RGBFloat {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl RGBFloat {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        check0to1!(r, g, b);
        Self { r, g, b }
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

impl From<&RGBFloat> for RGBInt {
    fn from(c: &RGBFloat) -> Self {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        // because RGBFloat r g b should be in [0..1]
        Self::new(
            (c.r * 255.0) as u8,
            (c.g * 255.0) as u8,
            (c.b * 255.0) as u8,
        )
    }
}

impl From<&RGBInt> for RGBFloat {
    fn from(c: &RGBInt) -> Self {
        let s = 1.0 / 255.0;
        Self::new(f64::from(c.r) * s, f64::from(c.g) * s, f64::from(c.b) * s)
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
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self::RGBI(RGBInt::new(r, g, b))
    }

    #[must_use]
    pub fn newf(r: f64, g: f64, b: f64) -> Self {
        Self::RGBF(RGBFloat::new(r, g, b))
    }

    #[must_use]
    pub fn i(&self) -> Cow<'_, RGBInt> {
        match self {
            Self::RGBF(c) => Cow::Owned(c.into()),
            Self::RGBI(c) => Cow::Borrowed(c),
        }
    }

    #[must_use]
    pub fn f(&self) -> Cow<'_, RGBFloat> {
        match self {
            Self::RGBF(c) => Cow::Borrowed(c),
            Self::RGBI(c) => Cow::Owned(c.into()),
        }
    }

    #[must_use]
    pub fn gradient(&self, rhs: &Self, slide: f64) -> Self {
        let a = (1.0 - slide) * self;
        let b = slide * rhs;
        let c1 = a.f();
        let c2 = b.f();
        Self::newf(c1.r + c2.r, c1.g + c2.g, c1.b + c2.b)
    }
}

impl Texture for Color {
    fn color(&self, _u: f64, _v: f64, _point: &Point3) -> Color {
        self.clone()
    }
}

impl Mul<&Color> for &Color {
    type Output = Color;
    fn mul(self, rhs: &Color) -> Self::Output {
        let c1 = self.f();
        let c2 = rhs.f();
        Color::newf(c1.r * c2.r, c1.g * c2.g, c1.b * c2.b)
    }
}

impl Mul<Color> for &Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        self * &rhs
    }
}

impl Mul<&Color> for Color {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Color> for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<f64> for &Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        let c = self.f();
        Color::newf(
            clamp(c.r * rhs, 0.0..=1.0),
            clamp(c.g * rhs, 0.0..=1.0),
            clamp(c.b * rhs, 0.0..=1.0),
        )
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl Mul<&Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: &Color) -> Self::Output {
        rhs * self
    }
}
impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        &rhs * self
    }
}
