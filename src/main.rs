// This is because of the Crate name.
// please use snake_case when possible
#![allow(non_snake_case)]

use cplex::Cplex;
use image::*;
use rayon::prelude::*;
use std::path::PathBuf;

mod colors;
mod cplex;

// This stuct is meant to bundle all the
// information for how we want the image to
// be normalized to the graph and produce an image
struct ImageMandelBrot {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,

    h: u32,
    w: u32,
}

// Other
// x1 = -2.0,   x2 = 1.0,    y1 = 1.0,    y2 = -1.0
// x1 = -0.2,   x2 = 0.0,    y1 = -0.8,   y2 = -1.0
// x1 = -0.1,   x2 = -0.05,  y1 = -0.8,   y2 = -0.85

fn main() {
    let start_time = std::time::Instant::now();

    let file_out = PathBuf::from(r"image\out\mandelbrot7.jpg");
    let size = 4;
    let mb = ImageMandelBrot {
        h: 1000 * size,
        w: 1000 * size,
        x1: -0.08,
        x2: -0.07,
        y1: -0.825,
        y2: -0.835,
    };

    let image = pixel_loop(&mb);
    image.save(file_out).expect("Could not save");

    println!("Total time elapsed {} ms", start_time.elapsed().as_millis());
}

fn pixel_loop(mb: &ImageMandelBrot) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let color_lookup: &Vec<Rgb<u8>> = &colors::color_gen(3);

    let data: Vec<Rgb<u8>> = (0..mb.h)
        .into_par_iter()
        .map(|y| {
            (0..mb.w).into_par_iter().map(move |x| {
                // i and j represent points on a graph
                let i = normalize(x, 0.0, mb.w as f32, mb.x1, mb.x2);
                let j = normalize(y, 0.0, mb.h as f32, mb.y1, mb.y2);

                let mut z = Cplex::new(0f32, 0f32);
                let c = Cplex::new(i, j);

                let max_iter = 500usize;

                // Complex Loop
                for t in 0..max_iter {
                    if z.Sq() > 4.0 {
                        // Apply color to everything that eventailly ends,
                        // If a point does converge it will be colored.
                        return color_lookup[t];
                    }
                    z.Pow2();
                    z.AddTo(&c);
                }

                // If the Complex Loop doesnt converge, return a Black color
                image::Rgb([0u8, 0u8, 0u8])
            })
        })
        .flatten()
        .collect();

    // Generate All of the X Y pairs
    /*let mut coords = Vec::new();
    for y in 0..mb.h {
        for x in 0..mb.w {
            coords.push((x, y));
        }
    }

    let data: Vec<Rgb<u8>> = coords
        .into_par_iter()
        .map(|(x, y)| {
            // i and j represent points on a graph
            let i = normalize(x, 0.0, mb.w as f32, mb.x1, mb.x2);
            let j = normalize(y, 0.0, mb.h as f32, mb.y1, mb.y2);

            /*let mut z = Complex::new(0f32, 0f32);
            let c = Complex::new(i, j);*/

            let mut z = Cplex::new(0f32, 0f32);
            let c = Cplex::new(i, j);

            //let mut total_iter = 0usize;
            let max_iter = 500usize;

            // Complex Loop
            for t in 0..max_iter {
                if z.Sq() > 4.0 {
                    // Apply color to everything that eventailly ends,
                    // If a point does converge it will be colored.
                    return color_lookup[t];
                }

                //z = z * z + c;
                z.Pow2();
                z.AddTo(&c);
            }

            // If the Complex Loop doesnt converge, return a Black color
            image::Rgb([0u8, 0u8, 0u8])
        })
        .collect();*/

    // Swap move the pixals into the image
    let mut rgb = image::DynamicImage::new_rgb8(mb.w, mb.h).into_rgb8();
    rgb.pixels_mut().enumerate().for_each(|(i, p)| *p = data[i]);

    rgb
}

fn normalize<T, I: 'static>(value: T, from_min: I, from_max: I, to_min: I, to_max: I) -> I
where
    T: num::cast::AsPrimitive<I>,
    I: Copy + num::Num,
{
    to_min + ((value.as_() - from_min) * (to_max - to_min)) / (from_max - from_min)
}

// Averaging general runs
// Go   Par time --  1,786 ms
// Rust Par time --  1,625 ms
