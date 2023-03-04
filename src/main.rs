use std::cmp::min;

use image::{Rgb, ImageBuffer, RgbImage};

fn main() {
    const MAX_SEARCH_LENGTH:u32 = 5000;
    const RESOLUTION:f64 = 0.001; // lower is better
    const X_BOUND:f64 = 10.0;
    const INITIAL_POP:f64 = 0.5; // amount of population alive(0.5 = 50%)

    let mut r:f64 = 0.0;
    let mut x:f64;
    
    let mut log_values:Vec<Vec<f64>> = Vec::new();
    
    while r <= X_BOUND {
        x = INITIAL_POP;
        let mut iter = 0;
        let mut vals:Vec<f64> = Vec::new();

        while iter < MAX_SEARCH_LENGTH{
            for _ in 0..500 {
                x = logistic(r, x);
            }
            x = (x * 1000.0).round() / 1000.0;
            if vals.contains(&x) {
                // println!("found");
                break;
            } else {
                vals.push(x);
            }

            x = logistic(r, x);
            iter += 1;
        }

        // println!("R: {}, Vals: {:?}", r, vals);

        log_values.push(vals);

        r += 0.001;
        r = (r * 1000.0).round() / 1000.0;
    }

    // Draw image
    let mut img = RgbImage::new((X_BOUND/RESOLUTION) as u32,(1.0/RESOLUTION) as u32);
    
    for (i, r_vals) in log_values.iter().enumerate() {
        for val in r_vals {
            // println!("Put pixel {}, {}", i, val/RESOLUTION);
            img.put_pixel(min(i as u32,((X_BOUND/RESOLUTION) as u32) - 1), (val/RESOLUTION) as u32, Rgb([255,255,255]))
        }
    }

    img.save(format!("output.png")).unwrap();


}

fn logistic(rate: f64, pop: f64) -> f64{
    rate * pop * (1.0 - pop)
}