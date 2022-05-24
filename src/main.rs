use ray_tracing::utils;
use utils::ppm::PPM;
use utils::ppm::RGBTriplet;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

fn main() {
    let mut ppm = PPM::new(WIDTH, HEIGHT);    
    
    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let r = ((i as f64 / 255.0) * 255.999) as u8;
            let g = ((j as f64 / 255.0) * 255.999) as u8;
            let b = (0.25 * 255.999) as u8;
            
            ppm.set(i, HEIGHT - j - 1, RGBTriplet::new(r, g, b));
        }
    }

    println!("{:?}", ppm);
}