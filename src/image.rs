use {
    crate::prelude::*,
    log::info,
    std::{
        fs::File,
        io::{BufWriter, Write},
        iter::FromIterator,
        ops::{Index, IndexMut},
        path::Path,
    },
    rayon::prelude::*,
};

#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
    colors: Vec<Color>,
}

impl Image {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        let colors = vec![Color::default(); width * height];
        Self {
            width,
            height,
            colors,
        }
    }

    /// # Errors
    /// When open or write to file failed
    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        write!(
            &mut file,
            "P3\n{width} {height}\n255\n",
            width = self.width,
            height = self.height
        )?;

        for row in 0..self.height {
            for column in 0..self.width {
                let index = row * self.width + column;
                let color = &self.colors[index].i();
                writeln!(
                    &mut file,
                    "{r} {g} {b}",
                    r = color.r,
                    g = color.g,
                    b = color.b
                )?;
            }
        }

        Ok(())
    }

    /// # Errors
    ///
    /// When image pixel count is not divisible by new width
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
        Self {
            height: 1,
            width: colors.len(),
            colors,
        }
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

#[derive(Debug)]
pub struct Painter {
    pub width: usize,
    pub height: usize,
    samples: usize,
}

impl Painter {
    #[must_use]
    pub const fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            samples: 1,
        }
    }

    #[must_use]
    pub const fn set_samples(mut self, samples: usize) -> Self {
        self.samples = samples;
        self
    }

    #[allow(clippy::cast_precision_loss)] // because row and column is small enough in practice
    fn calculate_uv(&self, row: usize, column: usize) -> (f64, f64) {
        let u = (column as f64 + Random::normal()) / self.width as f64;
        let v = ((self.height - 1 - row) as f64 + Random::normal()) / self.height as f64;
        (u, v)
    }

    /// # Errors
    ///
    /// When open or save to file failed
    pub fn draw<P, F>(&self, path: Option<P>, uv_color: F) -> std::io::Result<()>
    where
        P: AsRef<Path>,
        F: Fn(f64, f64) -> Vec3 + Send + Sync,
    {
        let mut file: BufWriter<Box<dyn Write>> = if let Some(path) = path {
            BufWriter::new(Box::new(File::create(path.as_ref())?))
        } else {
            BufWriter::new(Box::new(std::io::sink()))
        };
        write!(
            &mut file,
            "P3\n{width} {height}\n255\n",
            width = self.width,
            height = self.height
        )?;

        for row in 0..self.height {
            info!("Scan line remaining: {}", self.height - row);
            for column in 0..self.width {
                let color: Vec3 = (0..self.samples).into_par_iter()
                    .map(|_| {
                        let (u, v) = self.calculate_uv(row, column);
                        uv_color(u, v)
                    })
                    .sum();
                let color = color.into_color(self.samples);
                let color = color.i();
                writeln!(
                    &mut file,
                    "{r} {g} {b}",
                    r = color.r,
                    g = color.g,
                    b = color.b
                )?;
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
