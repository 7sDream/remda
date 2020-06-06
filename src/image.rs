use {
    log::info,
    std::{
        fs::File,
        io::{BufWriter, Write},
        iter::FromIterator,
        ops::{Index, IndexMut},
        path::Path,
    },
};

use super::color::Color;

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
        Vec::from_iter(iter).into()
    }
}

impl<T> From<T> for Image
where
    T: Into<Vec<Color>>,
{
    fn from(container: T) -> Self {
        let colors = container.into();
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

pub struct Painter {
    pub width: usize,
    pub height: usize,
}

impl Painter {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn draw<P, F>(&self, path: P, mut f: F) -> std::io::Result<()>
    where
        P: AsRef<Path>,
        F: FnMut(usize, usize) -> Color,
    {
        let mut file = BufWriter::new(File::create(path.as_ref())?);
        write!(&mut file, "P3\n{width} {height}\n255\n", width = self.width, height = self.height)?;

        for row in 0..self.height {
            info!("Scan line remaining: {}", self.height - row);
            for column in 0..self.width {
                let color = f(row, column);
                let color = color.i();
                write!(&mut file, "{r} {g} {b}\n", r = color.r, g = color.g, b = color.b)?;
            }
            // 16KB
            if file.buffer().len() >= 16 << 10 {
                file.flush()?;
            }
        }

        drop(file);

        Ok(())
    }
}
