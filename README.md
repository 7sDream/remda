# Remda

[《Ray Tracing in One Weekend》][book] in Rust.

## Screenshot

![][screenshot]

## Run

```bash
$ env RUST_LOG=info cargo run --release
```

Wait about 1 minute, you will get a `rendered.ppm` in current dir, that's your result.

If you want a bigger and clear image, adjust `height()` and `samples()` argument in `main.rs` and re-run.

The screenshot above(1920x1080, 512 samples) takes about 4 hours to render, be patient;

## LICENSE

CC0

[book]: https://raytracing.github.io/
[screenshot]: https://rikka.7sdre.am/files/a3618879-cf94-4ecd-b381-6b9d7e7f34a5.png
