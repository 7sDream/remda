# Remda

Learn [*Ray Tracing in One Weekend* series][book-series] using Rust.

Yet another naive and pure software ray tracing renderer.

## Render Result Examples

![rendered image of RTOW's final scene][rtow-final-scene-render-result]
*example rtow_13_1, 512 sample, 8 depth, 1920x1080, 8m30s*

![rendered image of RTNW's Cornell scene][rtnw-cornell-rotated-scene-render-result]
*example rtnw_8_2, 10240 sample, 10 depth, 1000x1000, 1h20m*

![rendered image of RTNW's Cornel smoke scene][rtnw-cornell-smoke-scene-render-result]
*example rtnw_9_2, 10240 sample, 10 depth, 1000x1000, 1h30m*

![rendered image of RTNW's final scene][rtnw-final-scene-render-result]
*example rtnw_10, 10240 sample, 10 depth, 1000x1000, 2h30m*

## Current Progress

- [x] [*Ray Tracing in One Weekend*][book-1]
  - [x] Basic types, Vec, Color, Ray, etc
  - [x] Background/Sky
  - [x] Sphere
  - [x] Lambertian Material
  - [x] Metal Material with Different Fuzz
  - [x] Glass Material with Different Refractive
  - [x] Pinhole Camera
  - [x] Camera Defocus/Depth Field
  - [x] (Extra) Parallelism (by using [rayon][rayon-crates-io])
- [x] [*Ray Tracing: The Next Week*][book-2]
  - [x] Motion Blur
  - [x] BVH(Bounding Volume Hierarchies)
  - [x] Solid Textures
  - [x] Perlin Noise
  - [x] Image Textures
  - [x] Rectangles
  - [x] Lights
  - [x] Cornell Box
  - [x] Box
  - [x] Instance Translation and Rotation
  - [x] Volumes/Participating Media
- [ ] [*Ray Tracing: The Rest of Your Life*][book-3] not started yet

## Run

Remda is a library crate, but you can run built-in examples(from the book series) to try it.

Use `cargo run --example` to get examples list, then choose one to run.

For example, to get final scene in section 13.1 of *Ray Tracing in One Weekend*, run

```bash
cargo run --example rtow_13_1 --release
```

Wait about 1s(according to your machine's CPU performance), you will get a `rtow_13_1.ppm` in current dir, that's your result.

If you want a bigger and clear image, adjust `height()`, `depth` and `samples()` parameter in example source file and re-run.

You can also try other examples if you want.

PS: Pure software ray tracing takes a long time to render, be patient.

## LICENSE

GPLv3

Except:

- `example/earth-map.png`, download from [NASA][earth-map-source], fall in public domain.

[book-series]: https://raytracing.github.io/
[book-1]: https://raytracing.github.io/books/RayTracingInOneWeekend.html
[book-2]: https://raytracing.github.io/books/RayTracingTheNextWeek.html
[book-3]: https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html
[rtow-final-scene-render-result]: https://rikka.7sdre.am/files/a952c7ca-af57-46a6-959f-237702333ab6.png
[rtnw-cornell-rotated-scene-render-result]: https://rikka.7sdre.am/files/1721b196-b746-4e6d-a4d0-f9c7c2e75c41.png
[rtnw-cornell-smoke-scene-render-result]: https://rikka.7sdre.am/files/545972fd-d10d-4345-9e8c-3ba16fb50524.png
[rtnw-final-scene-render-result]: https://rikka.7sdre.am/files/3e1e1849-54bf-4a7b-9e09-b2cc25b5cf6f.png
[rayon-crates-io]: https://crates.io/crates/rayon
[earth-map-source]: http://visibleearth.nasa.gov/view.php?id=57752
