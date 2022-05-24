use indicatif::ProgressStyle;
use ray_tracing::utils;
use utils::ppm::PPM;
use utils::ppm::RGBTriplet;
use indicatif::ProgressBar;

const WIDTH: u64 = 256;
const HEIGHT: u64 = 256;

fn main() {
    let bar = ProgressBar::new(WIDTH * HEIGHT);
    bar.set_draw_delta((WIDTH * HEIGHT) / 100);
    bar.set_style(ProgressStyle::default_bar()
        .template("LOADING PPM: {percent}% {bar:40.cyan/blue} {pos:>7}/{len:7}"));

    let mut ppm = PPM::new(WIDTH, HEIGHT);
    
    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            bar.inc(1);
            
            ppm.set(i, HEIGHT - j - 1, RGBTriplet::new(
                    ((i as f64 / 255.0) * 255.999) as u8, 
                    ((j as f64 / 255.0) * 255.999) as u8, 
                    (0.25 * 255.999) as u8,
            ));
        }
    }
    bar.finish();

    println!("{:?}", ppm);
}
