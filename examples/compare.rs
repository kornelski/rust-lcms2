
use lcms2::*;

use std::env;

fn main() {
    let path = env::args().nth(1).expect("Specify a profile as an argument");
    let p1 = Profile::new_file(path).unwrap();
    let p2 = Profile::new_srgb();

    let t = Transform::new(&p1, PixelFormat::RGB_8, &p2, PixelFormat::RGB_8, Intent::Perceptual).unwrap();

    let mut total_diff = 0;
    let mut max_diff = 0;
    let mut n = 0;

    for r in (0..256).into_iter().step_by(3) {
        for g in (0..256).into_iter().step_by(4) {
            for b in (0..256).into_iter().step_by(5) {
                let input = [(r as u8, g as u8, b as u8)];
                let mut out = [(0, 0, 0)];
                t.transform_pixels(&input, &mut out);

                n += 1;
                if input != out {
                    let (r2,g2,b2) = out[0];
                    let diff = (r as i32 - r2 as i32).pow(2) +
                               (g as i32 - g2 as i32).pow(2) +
                               (b as i32 - b2 as i32).pow(2);
                    total_diff += diff as usize;
                    if diff > max_diff {
                        max_diff = diff;
                    }
                    println!("{:02X}{:02X}{:02X} => {:02X}{:02X}{:02X} (off by {})", r,g,b, r2,g2,b2, diff);
                }
            }
        }
    }

    println!("Average squared difference from sRGB: {:.5}. Max {}.", total_diff as f64 / n as f64, max_diff);
}
