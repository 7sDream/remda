use {
    crate::{prelude::*, texture::Texture},
    image::{DynamicImage, GenericImageView},
    std::{
        fmt::{Debug, Formatter},
        path::Path,
    },
};

pub struct Image {
    img: DynamicImage,
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Image {{ {}x{} }} ",
            self.img.width(),
            self.img.height()
        ))
    }
}

impl Image {
    /// # Errors
    ///
    /// When load image failed
    pub fn new<P: AsRef<Path>>(p: P) -> Result<Self, String> {
        let img = image::open(p).map_err(|e| e.to_string())?;
        Ok(Self { img })
    }
}

impl Texture for Image {
    #[allow(clippy::cast_sign_loss)] // u v and width all non-negative
    fn color(&self, u: f64, v: f64, _point: &Point3) -> Color {
        let v = 1.0 - v;
        let mut px = (u * f64::from(self.img.width())) as u32;
        let mut py = (v * f64::from(self.img.height())) as u32;
        if px >= self.img.width() {
            px = self.img.width() - 1;
        }
        if py >= self.img.height() {
            py = self.img.height() - 1;
        }

        let color = self.img.get_pixel(px, py);
        Color::new_int(color.0[0], color.0[1], color.0[2])
    }
}
