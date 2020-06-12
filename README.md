# Remda

Learn [《Ray Tracing in One Weekend》series][book-series] using Rust.

Yet another simple and pure software ray tracing renderer.

## Screenshot

![][screenshot]

## Current Progress

- [ ] [《Ray Tracing in One Weekend》][book-1]
    - [x] Basic types, Vec, Color, Ray, etc
    - [x] Background/Sky
    - [x] Sphere
    - [x] Lambertian Material
    - [x] Metal Material with Different Fuzz
    - [x] Glass Material with Different Refractive
    - [x] Pinhole Camera
    - [x] Camera Defocus/Depth Field
    - [ ] Triangles Geometry
    - [x] Parallelism (by using [rayon][rayon-crates-io])
- [ ] [《Ray Tracing: The Next Week》][book-2]
    - [ ] Motion Blur
    - [ ] BVH(Bounding Volume Hierarchies)
    - [ ] Solid Textures
    - [ ] Perlin Noise
    - [ ] Image Textures
    - [ ] Rectangles
    - [ ] Lights
    - [ ] Box
    - [ ] Object Translation
    - [ ] Volumes
- [ ] [《Ray Tracing: The Rest of Your Life》][book-3] not started yet

## Run

```bash
$ cargo run --release
```

Wait about 10s(according to your machine's CPU performance), you will get a `rendered.ppm` in current dir, that's your result.

If you want a bigger and clear image, adjust `height()` and `samples()` argument in `main.rs` and re-run.

The screenshot above takes about 1 hours to render(1920x1080, 512 samples, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz), be patient;

## LICENSE

GPLv3

[book-series]: https://raytracing.github.io/
[book-1]: https://raytracing.github.io/books/RayTracingInOneWeekend.html
[book-2]: https://raytracing.github.io/books/RayTracingTheNextWeek.html
[book-3]: https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html
[screenshot]: https://rikka.7sdre.am/files/bb651ecd-a6d4-4f88-bf81-98757390eb60.png
[rayon-crates-io]: https://crates.io/crates/rayon
