use {
    super::scene,
    remda::{
        camera::{Camera, CameraBuilder},
        hittable::{
            collection::HittableList,
            transform::{AARotation, ByYAxis, Translation},
            AARect, AARectMetrics, Carton,
        },
        material::{DiffuseLight, Lambertian},
        prelude::*,
    },
};

#[must_use]
fn motion_blur_world(seed: Option<u64>, checker: bool) -> HittableList {
    scene::world(seed, true, checker)
}

#[must_use]
fn motion_blur_camera() -> Camera {
    scene::camera(true)
}

#[must_use]
pub fn motion_blur(seed: Option<u64>, checker: bool) -> (Camera, HittableList) {
    (motion_blur_camera(), motion_blur_world(seed, checker))
}

#[must_use]
pub fn cornell_box(carton: bool, carton_rotation: bool) -> (Camera, HittableList) {
    let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let light = DiffuseLight::new(Color::new(1.0, 1.0, 1.0)).multiplier(15.0);

    let mut objects = HittableList::default();

    objects
        .add(AARect::new_yz(
            AARectMetrics::new(555.0, (0.0, 555.0), (0.0, 555.0)),
            green,
        ))
        .add(AARect::new_yz(
            AARectMetrics::new(0.0, (0.0, 555.0), (0.0, 555.0)),
            red,
        ))
        .add(AARect::new_xz(
            AARectMetrics::new(554.0, (213.0, 343.0), (227.0, 332.0)),
            light,
        ))
        .add(AARect::new_xz(
            AARectMetrics::new(0.0, (0.0, 555.0), (0.0, 555.0)),
            white.clone(),
        ))
        .add(AARect::new_xz(
            AARectMetrics::new(555.0, (0.0, 555.0), (0.0, 555.0)),
            white.clone(),
        ))
        .add(AARect::new_xy(
            AARectMetrics::new(555.0, (0.0, 555.0), (0.0, 555.0)),
            white.clone(),
        ));

    if carton {
        if carton_rotation {
            objects
                .add(Translation::new(
                    AARotation::<ByYAxis, _>::new(
                        Carton::new(
                            Point3::new(0.0, 0.0, 0.0),
                            Point3::new(165.0, 165.0, 165.0),
                            white.clone(),
                        ),
                        -18.0,
                    ),
                    Vec3::new(130.0, 0.0, 65.0),
                ))
                .add(Translation::new(
                    AARotation::<ByYAxis, _>::new(
                        Carton::new(
                            Point3::new(0.0, 0.0, 0.0),
                            Point3::new(165.0, 330.0, 165.0),
                            white,
                        ),
                        15.0,
                    ),
                    Vec3::new(265.0, 0.0, 295.0),
                ));
        } else {
            objects
                .add(Carton::new(
                    Point3::new(130.0, 0.0, 65.0),
                    Point3::new(295.0, 165.0, 230.0),
                    white.clone(),
                ))
                .add(Carton::new(
                    Point3::new(265.0, 0.0, 295.0),
                    Point3::new(430.0, 330.0, 460.0),
                    white,
                ));
        }
    }

    let camera = CameraBuilder::default()
        .aspect_ratio(1.0)
        .fov(40.0)
        .look_from(Point3::new(278.0, 278.0, -800.0))
        .look_at(Point3::new(278.0, 278.0, 0.0))
        .build();

    (camera, objects)
}
