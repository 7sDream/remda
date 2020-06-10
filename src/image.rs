use {
    crate::prelude::*,
    log::info,
    rayon::prelude::*,
    std::{
        cmp::Reverse,
        collections::BinaryHeap,
        fs::File,
        io::{BufWriter, Write},
        iter::FromIterator,
        ops::{Index, IndexMut},
        path::Path,
        sync::{
            atomic::{AtomicBool, Ordering},
            mpsc::{channel, Receiver},
        },
    },
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

    fn write_file(
        &self, path: Option<&Path>, rx: Receiver<(usize, String)>,
    ) -> std::io::Result<()> {
        let mut file: BufWriter<Box<dyn Write>> = if let Some(path) = path {
            BufWriter::new(Box::new(File::create(&path)?))
        } else {
            BufWriter::new(Box::new(std::io::sink()))
        };

        write!(
            &mut file,
            "P3\n{width} {height}\n255\n",
            width = self.width,
            height = self.height
        )?;

        let mut pixels: BinaryHeap<Reverse<(usize, String)>> = BinaryHeap::new();
        let mut current: usize = 0;
        let mut line = 0;
        for pixel in rx {
            pixels.push(Reverse(pixel));
            while let Some(&Reverse((idx, ref pixel))) = pixels.peek() {
                if idx != current {
                    break;
                }
                writeln!(&mut file, "{}", pixel)?;
                pixels.pop();
                current += 1;
                if current / self.width > line {
                    line += 1;
                    info!("Scan line remaining: {}", self.height - line);
                }
            }
            if file.buffer().len() > 64 << 10 {
                file.flush()?;
            }
        }

        file.flush()?;
        drop(file);

        Ok(())
    }

    /// # Errors
    ///
    /// When open or save to file failed
    pub fn draw<P, F>(&self, path: Option<P>, uv_color: F) -> std::io::Result<()>
    where
        P: AsRef<Path>,
        F: Fn(f64, f64) -> Vec3 + Send + Sync,
    {
        let (tx, rx) = channel();

        let cancel = AtomicBool::new(false);
        let mut result = std::io::Result::Ok(());

        let path = match path {
            Some(ref path) => Some(path.as_ref()),
            None => None,
        };

        rayon::ThreadPoolBuilder::default()
            .num_threads(num_cpus::get() + 1)
            .build_global()
            .unwrap();
        info!("Worker Thread Count: {}", rayon::current_num_threads());

        rayon::scope(|s| {
            s.spawn(|_| {
                (0..self.height)
                    .into_par_iter()
                    .for_each_with(tx, |sender, row| {
                        (0..self.width).for_each(|column| {
                            if cancel.load(std::sync::atomic::Ordering::Relaxed) {
                                return;
                            }
                            let color: Vec3 = (0..self.samples)
                                .map(|_| {
                                    let (u, v) = self.calculate_uv(row, column);
                                    uv_color(u, v)
                                })
                                .sum();
                            let color = color.into_color(self.samples);
                            let color = color.i();
                            let idx = row * self.width + column;
                            sender
                                .send((
                                    idx,
                                    format!("{r} {g} {b}", r = color.r, g = color.g, b = color.b),
                                ))
                                .unwrap();
                        });
                    });
            });
            s.spawn(|_| {
                result = self.write_file(path, rx);
                if result.is_err() {
                    cancel.store(true, Ordering::Relaxed);
                }
            })
        });

        result
    }
}
