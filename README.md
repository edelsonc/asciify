# asciify
Turn any image into ascii art!

This is a small command line application that turns images into ascii art. Written in rust, it is fast and easy to build with `cargo`.

# Getting Started
First, you'll need an installation of [rust](https://www.rust-lang.org/en-US/). After installing rust, clone this reposity and build the application using `cargo`

```
$ git clone https://github.com/edelsonc/asciify
$ cd asciify
$ cargo build --release
```

A new directory named `target` will be created, with a `release` directory. Your executable will be there.

# Examples
An example image and output are provided above as `pixel_house.gif` and `pixel_house.txt`.

The example was created using the following
```
$ asciify pixel_house.gif -s 200 100 > pixel_house.txt
```

This leverages the `-s` flag, which resized the image. Additionally, note that the example is very large. You may have to zoom out to prevent line wrapping.
