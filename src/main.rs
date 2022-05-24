use indicatif::ProgressStyle;
use ray_tracing::utils;
use utils::ppm::PPM;
use utils::ppm::RGBTriplet;
use indicatif::ProgressBar;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

fn main() {
    let bar = ProgressBar::new((WIDTH * HEIGHT) as u64);
    bar.set_draw_delta(((WIDTH * HEIGHT) / 100) as u64);
    bar.set_style(ProgressStyle::default_bar()
        .template("LOADING PPM: {percent}% {bar:40.cyan/blue} {pos:>7}/{len:7}"));

    let mut ppm = PPM::new(WIDTH, HEIGHT);    
    
    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            bar.inc(1);

            let r = ((i as f64 / 255.0) * 255.999) as u8;
            let g = ((j as f64 / 255.0) * 255.999) as u8;
            let b = (0.25 * 255.999) as u8;
            
            ppm.set(i, HEIGHT - j - 1, RGBTriplet::new(r, g, b));
        }
    }
    bar.finish();

    println!("{:?}", ppm);
}