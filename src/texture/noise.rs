use {super::Texture, crate::prelude::*};

#[derive(Debug, Clone)]
pub struct Perlin {
    point_count: usize,
    ran_float: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    #[must_use]
    pub fn new(point_count: usize) -> Self {
        let ran_float = (0..point_count).map(|_| Random::normal()).collect();
        let mut perm_x = (0..point_count).collect();
        Random::shuffle(&mut perm_x);
        let mut perm_y = (0..point_count).collect();
        Random::shuffle(&mut perm_y);
        let mut perm_z = (0..point_count).collect();
        Random::shuffle(&mut perm_z);
        Self {
            point_count,
            ran_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }
}

impl Texture for Perlin {
    #[allow(clippy::cast_sign_loss)] // because we do abs before cast
    #[allow(clippy::cast_possible_truncation)] // truncation is expected behavior
    fn color(&self, _u: f64, _v: f64, point: &Point3) -> Color {
        let i = ((4.0 * point.x).abs() as usize) & (self.point_count - 1);
        let j = ((4.0 * point.y).abs() as usize) & (self.point_count - 1);
        let k = ((4.0 * point.z).abs() as usize) & (self.point_count - 1);

        let radio = self.ran_float[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]];
        Color::newf(1.0, 1.0, 1.0) * radio
    }
}
