use {
    super::common,
    remda::{
        camera::{Camera, CameraBuilder},
        geometry::{AARect, AARectGeometry, GeometryList},
        material::{DiffuseLight, Lambertian},
        prelude::*,
    },
};

#[must_use]
fn motion_blur_world(seed: Option<u64>, checker: bool) -> GeometryList {
    common::world(seed, true, checker)
}

#[must_use]
fn motion_blur_camera() -> Camera {
    common::camera(true)
}

#[must_use]
pub fn motion_blur(seed: Option<u64>, checker: bool) -> (Camera, GeometryList) {
    (motion_blur_camera(), motion_blur_world(seed, checker))
}

#[must_use]
pub fn cornell_box(_flip: bool) -> (Camera, GeometryList) {
    let red = Lambertian::new(Color::newf(0.65, 0.05, 0.05));
    let green = Lambertian::new(Color::newf(0.12, 0.45, 0.15));
    let white = Lambertian::new(Color::newf(0.73, 0.73, 0.73));
    let light = DiffuseLight::new(Color::newf(1.0, 1.0, 1.0)).multiplier(15.0);

    let mut objects = GeometryList::default();

    objects
        .add(AARect::new_yz(
            AARectGeometry::new(555.0, (0.0, 555.0), (0.0, 555.0)),
            green,
        ))
        .add(AARect::new_yz(
            AARectGeometry::new(0.0, (0.0, 555.0), (0.0, 555.0)),
            red,
        ))
        .add(AARect::new_xz(
            AARectGeometry::new(554.0, (213.0, 343.0), (227.0, 332.0)),
            light,
        ))
        .add(AARect::new_xz(
            AARectGeometry::new(0.0, (0.0, 555.0), (0.0, 555.0)),
            white.clone(),
        ))
        .add(AARect::new_xz(
            AARectGeometry::new(555.0, (0.0, 555.0), (0.0, 555.0)),
            white.clone(),
        ))
        .add(AARect::new_xy(
            AARectGeometry::new(555.0, (0.0, 555.0), (0.0, 555.0)),
            white,
        ));

    let camera = CameraBuilder::default()
        .aspect_ratio(1.0)
        .fov(40.0)
        .look_from(Point3::new(278.0, 278.0, -800.0))
        .look_at(Point3::new(278.0, 278.0, 0.0))
        .build();

    (camera, objects)
}
