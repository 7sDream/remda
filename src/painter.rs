use {
    crate::{internal::rayon_seq_iter::SeqForEach, prelude::*},
    log::info,
    rayon::{prelude::*, ThreadPool, ThreadPoolBuilder},
    std::{
        fs::File,
        io::{BufWriter, Write},
        iter::FromIterator,
        ops::{Index, IndexMut},
        path::Path,
        sync::atomic::{AtomicBool, Ordering},
    },
};

#[derive(Debug)]
pub struct PPMImage {
    width: usize,
    height: usize,
    colors: Vec<Color>,
}

impl PPMImage {
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
    pub fn reshape(&mut self, width: usize) -> Result<(), &'static str> {
        if self.colors.len() % width == 0 {
            self.width = width;
            self.height = self.colors.len() / width;
            Ok(())
        } else {
            Err("Shape invalid")
        }
    }
}

impl FromIterator<Color> for PPMImage {
    fn from_iter<T: IntoIterator<Item = Color>>(iter: T) -> Self {
        Vec::from_iter(iter).into()
    }
}

impl<T> From<T> for PPMImage
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

impl Index<(usize, usize)> for PPMImage {
    type Output = Color;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        self.index(row * self.width + col)
    }
}

impl Index<usize> for PPMImage {
    type Output = Color;
    fn index(&self, index: usize) -> &Self::Output {
        self.colors.index(index)
    }
}

impl IndexMut<(usize, usize)> for PPMImage {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        self.index_mut(row * self.width + col)
    }
}

impl IndexMut<usize> for PPMImage {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.colors.index_mut(index)
    }
}

#[derive(Debug)]
pub struct Painter {
    pub width: usize,
    pub height: usize,
    samples: usize,
    gamma: bool,
    threads: usize,
    parallel: bool,
}

struct PainterOutputContext<'c> {
    file: BufWriter<Box<dyn Write>>,
    cancel: &'c AtomicBool,
}

impl Painter {
    #[must_use]
    pub const fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            gamma: true,
            samples: 50,
            threads: 0,
            parallel: true,
        }
    }

    #[must_use]
    pub const fn gamma(mut self, gamma: bool) -> Self {
        self.gamma = gamma;
        self
    }

    #[must_use]
    pub const fn samples(mut self, samples: usize) -> Self {
        self.samples = samples;
        self
    }

    #[must_use]
    pub const fn threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }

    #[must_use]
    pub const fn parallel(mut self, parallel: bool) -> Self {
        self.parallel = parallel;
        self
    }

    #[allow(clippy::cast_precision_loss)] // because row and column is small enough in practice
    fn calculate_uv(&self, row: usize, column: usize) -> (f64, f64) {
        if self.samples == 1 {
            let u = (column as f64) / self.width as f64;
            let v = ((self.height - 1 - row) as f64) / self.height as f64;
            (u, v)
        } else {
            let u = (column as f64 + Random::normal()) / self.width as f64;
            let v = ((self.height - 1 - row) as f64 + Random::normal()) / self.height as f64;
            (u, v)
        }
    }

    fn create_output_file(
        &self, path: Option<&Path>,
    ) -> std::io::Result<BufWriter<Box<dyn Write>>> {
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

        Ok(file)
    }

    fn create_output_context<'c>(
        &self, path: Option<&Path>, cancel: &'c AtomicBool,
    ) -> std::io::Result<PainterOutputContext<'c>> {
        let file = self.create_output_file(path)?;
        Ok(PainterOutputContext { file, cancel })
    }

    // TODO: make it return RGBInt type
    fn render_pixel<F>(&self, row: usize, column: usize, uv_color: &F) -> (u8, u8, u8)
    where
        F: Fn(f64, f64) -> Vec3 + Send + Sync,
    {
        let color: Vec3 = (0..self.samples)
            .map(|_| {
                let (u, v) = self.calculate_uv(row, column);
                uv_color(u, v)
            })
            .sum();
        let color = color.into_color(self.samples, self.gamma);
        let color = color.i();
        (color.r, color.g, color.b)
    }

    fn parallel_render_row<F>(
        &self, row: usize, uv_color: &F, cancel: &AtomicBool,
    ) -> Vec<(u8, u8, u8)>
    where
        F: Fn(f64, f64) -> Vec3 + Send + Sync,
    {
        (0..self.width)
            .map(|column| {
                if cancel.load(Ordering::Relaxed) {
                    return (0, 0, 0);
                }
                self.render_pixel(row, column, &uv_color)
            })
            .collect::<Vec<_>>()
    }

    fn seq_render_row<F>(&self, row: usize, uv_color: &F) -> Vec<(u8, u8, u8)>
    where
        F: Fn(f64, f64) -> Vec3 + Send + Sync,
    {
        (0..self.width)
            .map(|column| self.render_pixel(row, column, &uv_color))
            .collect::<Vec<_>>()
    }

    fn parallel_render_row_iter<'c, F>(
        &'c self, uv_color: F, cancel: &'c AtomicBool,
    ) -> impl IndexedParallelIterator<Item = Vec<(u8, u8, u8)>> + 'c
    where
        F: Fn(f64, f64) -> Vec3 + Send + Sync + 'c,
    {
        (0..self.height)
            .into_par_iter()
            .map(move |row| self.parallel_render_row(row, &uv_color, cancel))
    }

    fn seq_render_row_iter<'c, F>(
        &'c self, uv_color: F,
    ) -> impl Iterator<Item = Vec<(u8, u8, u8)>> + 'c
    where
        F: Fn(f64, f64) -> Vec3 + Send + Sync + 'c,
    {
        (0..self.height).map(move |row| self.seq_render_row(row, &uv_color))
    }

    fn real_row_pixels_to_file(
        context: &mut PainterOutputContext<'_>, pixels: Vec<(u8, u8, u8)>,
    ) -> std::io::Result<()> {
        for pixel in pixels {
            writeln!(context.file, "{} {} {}", pixel.0, pixel.1, pixel.2)?;
        }
        context.file.flush()
    }

    fn row_pixels_to_file(
        &self, context: &mut PainterOutputContext<'_>, row: usize, pixels: Vec<(u8, u8, u8)>,
    ) -> std::io::Result<()> {
        info!("Scan line remaining: {}", self.height - row);
        Self::real_row_pixels_to_file(context, pixels).map_err(|e| {
            context.cancel.store(true, Ordering::Relaxed);
            e
        })
    }

    fn parallel_render_and_output<F>(&self, uv_color: F, path: Option<&Path>) -> std::io::Result<()>
    where
        F: Fn(f64, f64) -> Vec3 + Send + Sync,
    {
        let cancel = AtomicBool::new(false);

        self.parallel_render_row_iter(uv_color, &cancel)
            .seq_for_each_with(
                || self.create_output_context(path, &cancel),
                |context, row, pixels| self.row_pixels_to_file(context, row, pixels),
            )
    }

    fn setup_thread_pool(&self) -> std::io::Result<ThreadPool> {
        let threads = if self.threads == 0 {
            num_cpus::get() + 1
        } else {
            self.threads + 1
        };
        ThreadPoolBuilder::default()
            .num_threads(threads)
            .build()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    /// # Errors
    ///
    /// When open or save to file failed
    pub fn draw<P, F>(&self, path: &Option<P>, uv_color: F) -> std::io::Result<()>
    where
        P: AsRef<Path>,
        F: Fn(f64, f64) -> Vec3 + Send + Sync,
    {
        let path = match path {
            Some(ref path) => Some(path.as_ref()),
            None => None,
        };

        if self.parallel {
            let pool = self.setup_thread_pool()?;

            info!("Worker thread count: {}", pool.current_num_threads());

            pool.install(|| self.parallel_render_and_output(uv_color, path))
        } else {
            let cancel = AtomicBool::new(false); // useless in parallel mode
            let mut context = self.create_output_context(path, &cancel)?;
            for (row, pixels) in self.seq_render_row_iter(uv_color).enumerate() {
                self.row_pixels_to_file(&mut context, row, pixels)?;
            }
            Ok(())
        }
    }
}
