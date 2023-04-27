use image::*;
use rayon::prelude::*;
use std::path::PathBuf;

mod colors;
mod cplex;

// This stuct is meant to bundle all the
// information for how we want the image to
// be normalized to the graph and produce an image
struct MandelBrotSettings {
    height: u32,
    width: u32,

    x: f64,
    y: f64,

    zoom: f64,
}

fn main() {
    let start_time = std::time::Instant::now();

    let size = 1.0;

    for iter in 0..=40 {
        // Set up Mandelbrot Settings
        let mbs = MandelBrotSettings {
            height: (1080.0 * size) as u32,
            width: (1920.0 * size) as u32,

            x: -0.76679001500011,
            y: -0.1000005000931,

            zoom: iter as f64,
        };

        // Generate the image
        let image = mandel_brot(&mbs);

        // Save the image with the specified file name
        let file_out = PathBuf::from(format!("image/out/zoom_{}.jpg", iter));

        image.save(file_out).expect("Could not save");
    }

    println!("Total time elapsed {} ms", start_time.elapsed().as_millis());
}

/// pixel_loop preforms the computation of mandelbrot image.
fn mandel_brot(settings: &MandelBrotSettings) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    // Generate lookup arrays
    let color_lookup = colors::color_gen(5);
    let normal_lookup = normal_gen(settings);

    // Iterates over normalized values and maps its coordiates to the complex graph
    let data: Vec<Rgb<u8>> = normal_lookup[1]
        .par_iter()
        .map(|&j| {
            normal_lookup[0].par_iter().map(move |&i| {
                let c = num::Complex::new(i, j);
                let mut z = num::Complex::new(0f64, 0f64);

                const MAX_ITER: usize = 500usize;

                // Complex Loop
                for t in color_lookup.iter().take(MAX_ITER) {
                    if cplex::sq(&z) > 4.0 {
                        return *t;
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
    let mut rgb = image::DynamicImage::new_rgb8(settings.width, settings.height).into_rgb8();

    rgb.pixels_mut().enumerate().for_each(|(i, p)| *p = data[i]);

    rgb
}

// The normalize function takes a value between a range of numbers and normalize
// it between a new range of numbers. For instance, if the range is 10..20 with
// a value of 15, and the new range is 0..100, the new value will be 50.
fn normalize<F, T: 'static>(value: F, from_min: T, from_max: T, to_min: T, to_max: T) -> T
where
    F: num::cast::AsPrimitive<T>,
    T: Copy + num::Num,
{
    to_min + ((value.as_() - from_min) * (to_max - to_min)) / (from_max - from_min)
}

// power_of_two is a helper function to calculate 2^x
// Ex: 2^3 = 8
// Ex: 2^4 = 16
// Ex: 2^5 = 32
fn power_of_two(x: u64) -> u64 {
    return 1 << x;
}

// normal_gen generates all possible X and Y value that could be iterated over.
// This is faster than creating a 2d array of all the pairs as every value is
// used more than once. For instance, keep track of the X value in the 2d array:
// 0,0  1,0  2,0
// 0,1  1,1  2,1
// 0,2  1,2  2,2
// Since X and Y are the same for there column/row, we can just store them in an array
fn normal_gen(settings: &MandelBrotSettings) -> [Vec<f64>; 2] {
    let cached_zoom = 1.0 / power_of_two(settings.zoom as u64) as f64;

    let arrx = (0..settings.width)
        .map(|x| {
            normalize(
                x,
                0.0,
                settings.width as f64,
                settings.x - cached_zoom,
                settings.x + cached_zoom,
            )
        })
        .collect();

    let arry = (0..settings.height)
        .map(|y| {
            normalize(
                y,
                0.0,
                settings.height as f64,
                settings.y - cached_zoom,
                settings.y + cached_zoom,
            )
        })
        .collect();

    [arrx, arry]
}
