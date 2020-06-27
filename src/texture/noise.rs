use {super::Texture, crate::prelude::*};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SmoothType {
    None,
    Interp,
    Hermitian,
}

#[derive(Debug, Clone)]
pub struct Perlin {
    point_count: usize,
    scale: f64,
    smooth: SmoothType,
    ran_float: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    #[must_use]
    pub fn new(point_count: usize, smooth: SmoothType) -> Self {
        let ran_float = (0..point_count).map(|_| Random::normal()).collect();
        let mut perm_x = (0..point_count).collect();
        Random::shuffle(&mut perm_x);
        let mut perm_y = (0..point_count).collect();
        Random::shuffle(&mut perm_y);
        let mut perm_z = (0..point_count).collect();
        Random::shuffle(&mut perm_z);
        Self {
            point_count,
            scale: 1.0,
            smooth,
            ran_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    #[must_use]
    pub fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }
}

#[allow(clippy::cast_precision_loss)] // i j k is small enough
fn interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    (0..2)
        .flat_map(move |i| {
            (0..2).flat_map(move |j| {
                (0..2).map(move |k| {
                    (i as f64).mul_add(u, (1 - i) as f64 * (1.0 - u))
                        * (j as f64).mul_add(v, (1 - j) as f64 * (1.0 - v))
                        * (k as f64).mul_add(w, (1 - k) as f64 * (1.0 - w))
                        * c[i][j][k]
                })
            })
        })
        .sum()
}

impl Texture for Perlin {
    #[allow(clippy::cast_sign_loss)] // because we do abs before cast
    #[allow(clippy::cast_possible_wrap)] // because di dj dk and point_count is small enough
    #[allow(clippy::cast_possible_truncation)] // truncation is expected behavior
    #[allow(clippy::cast_precision_loss)] // scene is not so big
    #[allow(clippy::many_single_char_names)]
    fn color(&self, _u: f64, _v: f64, point: &Point3) -> Color {
        let point = point * self.scale;
        Color::newf(1.0, 1.0, 1.0)
            * match self.smooth {
                SmoothType::None => {
                    let i = (((4.0 * point.x) as isize) & (self.point_count - 1) as isize) as usize;
                    let j = (((4.0 * point.y) as isize) & (self.point_count - 1) as isize) as usize;
                    let k = (((4.0 * point.z) as isize) & (self.point_count - 1) as isize) as usize;

                    self.ran_float[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
                }
                SmoothType::Interp | SmoothType::Hermitian => {
                    let i = point.x.floor() as isize;
                    let j = point.y.floor() as isize;
                    let k = point.z.floor() as isize;
                    let mut u = point.x - i as f64;
                    let mut v = point.y - j as f64;
                    let mut w = point.z - k as f64;

                    if self.smooth == SmoothType::Hermitian {
                        u = u * u * (3.0 - 2.0 * u);
                        v = v * v * (3.0 - 2.0 * v);
                        w = w * w * (3.0 - 2.0 * w);
                    }

                    let mut grays = [[[0.0; 2]; 2]; 2];

                    (0..2).for_each(|di| {
                        (0..2).for_each(|dj| {
                            (0..2).for_each(|dk| {
                                let xi =
                                    ((i + di as isize) & (self.point_count - 1) as isize) as usize;
                                let yi =
                                    ((j + dj as isize) & (self.point_count - 1) as isize) as usize;
                                let zi =
                                    ((k + dk as isize) & (self.point_count - 1) as isize) as usize;
                                let index = self.perm_x[xi] ^ self.perm_y[yi] ^ self.perm_z[zi];
                                grays[di][dj][dk] = self.ran_float[index];
                            })
                        })
                    });

                    interp(grays, u, v, w)
                }
            }
    }
}
