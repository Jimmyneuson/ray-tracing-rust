#[macro_use] extern crate itertools;

use ray_tracing::utils;
use utils::ppm::PPM;
use utils::ppm::RGBTriplet;
use utils::vector3::Vector3;
use utils::ray::Ray;

use indicatif::ProgressStyle;
use indicatif::ProgressBar;

fn ray_color(mut ray: Ray) -> RGBTriplet {
    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    let v = Vector3::lerp(&Vector3::new(1.0, 1.0, 1.0), 
                          &Vector3::new(0.5, 0.7, 1.0), t);
    RGBTriplet::new((v.x * 256.0) as u8, (v.y * 256.0) as u8, (v.z * 256.0) as u8)
}

fn main() {
    //let bar = ProgressBar::new(WIDTH * HEIGHT);
    //bar.set_draw_delta((WIDTH * HEIGHT) / 100);
    //bar.set_style(ProgressStyle::default_bar()
    //    .template("LOADING PPM: {percent}% {bar:40.cyan/blue} {pos:>7}/{len:7}"));


    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u64;

    let mut ppm = PPM::new(image_width, image_height);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 -
                            Vector3::new(0.0, 0.0, focal_length);
    
    
    for (j, i) in iproduct!((0..image_height).rev(), 0..image_width) {
        //bar.inc(1);
        
        let u = i as f64 / (image_width - 1) as f64;
        let v = j as f64 / (image_width - 1) as f64;
        let r = Ray::new(origin, lower_left_corner +
                                 u*horizontal + v*vertical
                                 - origin);
        
        ppm.set(i, image_height - j - 1, ray_color(r));
    }
    //bar.finish();

    println!("{:?}", ppm);
}
