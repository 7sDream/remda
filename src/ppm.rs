use std::{
    borrow::Cow,
    fs::File,
    io::Write,
    iter::FromIterator,
    ops::{Index, IndexMut},
    path::Path,
};

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
    pub const fn newi(r: u8, g: u8, b: u8) -> Self {
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

pub static BLACK: Color = Color::newi(0, 0, 0);
pub static WHITE: Color = Color::newi(255, 255, 255);
pub static RED: Color = Color::newi(255, 0, 0);
pub static GREEN: Color = Color::newi(0, 255, 0);
pub static BLUE: Color = Color::newi(0, 0, 255);

pub struct Image {
    width: usize,
    height: usize,
    colors: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let colors = vec![Color::default(); width * height];
        Self { width, height, colors }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        write!(&mut file, "P3\n{width} {height}\n255\n", width = self.width, height = self.height)?;

        for row in 0..self.height {
            for column in 0..self.width {
                let index = row * self.width + column;
                let color = &self.colors[index].i();
                write!(&mut file, "{r} {g} {b}\n", r = color.r, g = color.g, b = color.b)?;
            }
        }

        Ok(())
    }

    pub fn reshape(&mut self, width: usize) -> Result<(), ()> {
        if self.colors.len() % width == 0 {
            self.width = width;
            self.height = self.colors.len() / width;
            Ok(())
        } else {
            Err(())
        }
    }
}

impl FromIterator<Color> for Image {
    fn from_iter<T: IntoIterator<Item = Color>>(iter: T) -> Self {
        let colors = Vec::from_iter(iter);
        Self { height: 1, width: colors.len(), colors }
    }
}

impl Index<(usize, usize)> for Image {
    type Output = Color;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        self.index(row * self.width + col)
    }
}

impl Index<usize> for Image {
    type Output = Color;
    fn index(&self, index: usize) -> &Self::Output {
        self.colors.index(index)
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        self.index_mut(row * self.width + col)
    }
}

impl IndexMut<usize> for Image {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.colors.index_mut(index)
    }
}
