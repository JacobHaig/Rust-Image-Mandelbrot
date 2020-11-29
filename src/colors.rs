use image::*;

// HSV Converts a HSV value to a RGBu8 value
pub fn hsv(hue: f64, sat: f64, val: f64) -> Rgb<u8> {
    let hp = hue / 60.0;
    let col = val * sat;
    let x = col * (1.0 - num::abs(hp.rem_euclid(2.0) - 1.0));

    let m = val - col;
    let (mut red, mut green, mut blue) = (0.0, 0.0, 0.0);

    if (0.0..1.0).contains(&hp) {
        red = col;
        green = x;
    }
    if (1.0..2.0).contains(&hp) {
        green = col;
        red = x;
    }
    if (2.0..3.0).contains(&hp) {
        green = col;
        blue = x;
    }
    if (3.0..4.0).contains(&hp) {
        blue = col;
        green = x;
    }
    if (4.0..5.0).contains(&hp) {
        blue = col;
        red = x;
    }
    if (5.0..6.0).contains(&hp) {
        red = col;
        blue = x;
    }

    image::Rgb([
        ((m + red) * 255.0) as u8,
        ((m + green) * 255.0) as u8,
        ((m + blue) * 255.0) as u8,
    ])
}

pub fn color_gen(speed: i32) -> Vec<Rgb<u8>> {
    let mut colors: Vec<Rgb<u8>> = Vec::new();

    for i in 0..1000i32 {
        let angle = (i * speed + 360 / 3 * 2) as f64;
        let color = hsv(angle.rem_euclid(360.0), 1.0, 1.0);

        colors.push(color);
    }

    colors
}
