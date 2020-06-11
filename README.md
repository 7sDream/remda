# Remda

Learn [《Ray Tracing in One Weekend》series][book-series] using Rust.

Yet another simple and pure software ray tracing renderer.

## Screenshot

![][screenshot]

## Current Progress

- [ ] [《Ray Tracing in One Weekend》][book-1]
    - [x] Basic types, Vec, Color, Ray, etc
    - [x] Background/Sky
    - [x] Sphere Geometry
    - [x] Lambertian Material
    - [x] Metal Material with Different Fuzz
    - [x] Glass Material with Refractive
    - [x] Single Hole Camera
    - [x] Camera defocus/depth field
    - [ ] Lights
    - [ ] Triangles Geometry
    - [ ] Surface Textures
    - [ ] Solid textures
    - [ ] Volumes and Media
    - [x] Parallelism (by using [rayon][rayon-crates-io])
- [ ] [《Ray Tracing: The Next Week》][book-2] not started yet
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
[screenshot]: https://rikka.7sdre.am/files/a3618879-cf94-4ecd-b381-6b9d7e7f34a5.png
[rayon-crates-io]: https://crates.io/crates/rayon
