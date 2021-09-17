// This is because of the Crate name.
// please use snake_case when possible
#![allow(non_snake_case)]

use image::*;
use rayon::prelude::*;
use std::path::PathBuf;

mod colors;
mod cplex;

// This stuct is meant to bundle all the
// information for how we want the image to
// be normalized to the graph and produce an image
struct MandelBrotSettings {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,

    h: u32,
    w: u32,
}

// Other ImageMandelBrot settings
// x1 = -2.0,   x2 = 1.0,    y1 = 1.0,    y2 = -1.0
// x1 = -0.2,   x2 = 0.0,    y1 = -0.8,   y2 = -1.0
// x1 = -0.1,   x2 = -0.05,  y1 = -0.8,   y2 = -0.85

fn main() {
    let start_time = std::time::Instant::now();

    // Set up Mandelbrot Settings
    let file_out = PathBuf::from(r"image\out\mandelbrot2.jpg");
    let size = 2;
    let mbs = MandelBrotSettings {
        h: 1080 * size,
        w: 1920 * size,
        x1: -2.0,
        x2: 1.0,
        y1: 1.0,
        y2: -1.0,
    };

    let image = pixel_loop(&mbs);

    // Save the image with the specified file name
    image.save(file_out).expect("Could not save");

    println!("Total time elapsed {} ms", start_time.elapsed().as_millis());
}

/// pixel_loop preforms the computation of mandelbrot image.
/// # Arguments
///     Takes a referance to a `MandelbrotSettings` object.
/// # Returns
///     An `ImageBuffer` containing a Vec of u8s.
fn pixel_loop(mbs: &MandelBrotSettings) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    // Generate lookup arrays
    let color_lookup: &Vec<Rgb<u8>> = &colors::color_gen(3);
    let normal_lookup: &Vec<Vec<f32>> = &normal_gen(mbs);

    // Iterates over normalized values and maps its coordiates to the complex graph
    let data: Vec<Rgb<u8>> = normal_lookup[1]
        .par_iter()
        .map(|&j| {
            normal_lookup[0].par_iter().map(move |&i| {
                let c = num::Complex::new(i, j);
                let mut z = num::Complex::new(0f32, 0f32);

                let max_iter = 150usize;

                // Complex Loop
                for t in 0..max_iter {
                    if cplex::sq(&z) > 4.0 {
                        // Apply color to everything that eventailly ends,
                        // If a point does converge it will be colored.
                        return color_lookup[t];
                    }

                    cplex::mandel(&mut z, &c);
                }

                // If the Complex Loop doesnt converge, return a Black color
                image::Rgb([0u8, 0u8, 0u8])
            })
        })
        .flatten()
        .collect();

    // Create the Image and move the pixals into the image
    let mut rgb = image::DynamicImage::new_rgb8(mbs.w, mbs.h).into_rgb8();
    rgb.pixels_mut().enumerate().for_each(|(i, p)| *p = data[i]);

    rgb
}

// The normalize function takes a value between a range of numbers and normalize
// it between a new range of numbers. For instance, if the range is 10..20 with
// a value of 15, and the new range is 0..100, the new value will be 50.
fn normalize<T, I: 'static>(value: T, from_min: I, from_max: I, to_min: I, to_max: I) -> I
where
    T: num::cast::AsPrimitive<I>,
    I: Copy + num::Num,
{
    to_min + ((value.as_() - from_min) * (to_max - to_min)) / (from_max - from_min)
}

// normal_gen generates all possible X and Y value that could be iterated over.
// This is faster than creating a 2d array of all the pairs as every value is
// used more than once. For instance, keep track of the X value in the 2d array:
// 0,0  1,0  2,0
// 0,1  1,1  2,1
// 0,2  1,2  2,2
// Since X and Y are the same for there column/row, we can just store them in an array
fn normal_gen(mb: &MandelBrotSettings) -> Vec<Vec<f32>> {
    let mut arr: Vec<Vec<f32>> = vec![
        Vec::with_capacity(mb.w as usize), // X
        Vec::with_capacity(mb.h as usize), // Y
    ];

    for x in 0..mb.w as usize {
        arr[0].push(normalize(x, 0.0, mb.w as f32, mb.x1, mb.x2));
    }
    for y in 0..mb.h as usize {
        arr[1].push(normalize(y, 0.0, mb.h as f32, mb.y1, mb.y2));
    }
    arr
}
