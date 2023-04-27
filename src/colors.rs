use image::*;

// HSV Converts a HSV value to a RGBu8 value
pub fn hsv(hue: f64, sat: f64, val: f64) -> Rgb<u8> {
    let hp = hue / 60.0;
    let color = val * sat;
    let x = color * (1.0 - num::abs(hp.rem_euclid(2.0) - 1.0));

    let m = val - color;
    let (mut red, mut green, mut blue) = (0.0, 0.0, 0.0);

    if (0.0..1.0).contains(&hp) {
        red = color;
        green = x;
    }
    if (1.0..2.0).contains(&hp) {
        green = color;
        red = x;
    }
    if (2.0..3.0).contains(&hp) {
        green = color;
        blue = x;
    }
    if (3.0..4.0).contains(&hp) {
        blue = color;
        green = x;
    }
    if (4.0..5.0).contains(&hp) {
        blue = color;
        red = x;
    }
    if (5.0..6.0).contains(&hp) {
        red = color;
        blue = x;
    }

    image::Rgb([
        ((m + red) * 255.0) as u8,
        ((m + green) * 255.0) as u8,
        ((m + blue) * 255.0) as u8,
    ])
}

const COLOR_SIZE: usize = 1000;

#[allow(dead_code)]
pub fn color_gen(speed: usize) -> [Rgb<u8>; COLOR_SIZE] {
    let mut colors = [Rgb([0u8, 0u8, 0u8]); COLOR_SIZE];

    for i in 0..COLOR_SIZE {
        let angle = (i * speed + 360 / 3 * 2) as f64;
        let color: Rgb<u8> = hsv(angle.rem_euclid(360.0), 1.0, 0.80);

        colors[i as usize] = color;
    }

    colors
}
