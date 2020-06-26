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
    - [x] Motion Blur
    - [x] BVH(Bounding Volume Hierarchies)
    - [x] Solid Textures
    - [ ] Perlin Noise
    - [ ] Image Textures
    - [ ] Rectangles
    - [ ] Lights
    - [ ] Box
    - [ ] Object Translation
    - [ ] Volumes
- [ ] [《Ray Tracing: The Rest of Your Life》][book-3] not started yet

## Run

Remda is a library crate, but you can run built-in examples(from the book series) to try it.

Use `cargo run --example` to get examples list, then choose one to run.

For example, to get final scene in section 13.1 of《Ray Tracing in One Weekend》, run

```bash
$ cargo run --example rtow_13_1 --release
```

Wait about 1s(according to your machine's CPU performance), you will get a `rtow_13_1.ppm` in current dir, that's your result.

If you want a bigger and clear image, adjust `height()` and `samples()` argument in example source file and re-run.

PS: The screenshot above takes about 8min30s to render(1920x1080, 512 samples, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz), be patient;

You can also try other examples if you want.

## LICENSE

GPLv3

[book-series]: https://raytracing.github.io/
[book-1]: https://raytracing.github.io/books/RayTracingInOneWeekend.html
[book-2]: https://raytracing.github.io/books/RayTracingTheNextWeek.html
[book-3]: https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html
[screenshot]: https://rikka.7sdre.am/files/a952c7ca-af57-46a6-959f-237702333ab6.png
[rayon-crates-io]: https://crates.io/crates/rayon
