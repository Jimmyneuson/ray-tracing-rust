use ray_tracing::utils;

use utils::ppm::PPM;
use utils::ppm::RGBTriplet;
use utils::vector3::Vector3;
use utils::ray::Ray;
use utils::camera::Camera;

use indicatif::ProgressStyle;
use indicatif::ProgressBar;

use itertools::iproduct;

fn ray_color(mut ray: Ray) -> RGBTriplet {
    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    let v = Vector3::lerp(&Vector3::new(1.0, 1.0, 1.0), 
                          &Vector3::new(0.5, 0.7, 1.0), t).color();
    RGBTriplet::new(v.x as u8, v.y as u8, v.z as u8)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u64;
    
    // Set up
    let bar = ProgressBar::new(image_width * image_height);
    bar.set_draw_delta((image_width * image_height) / 100);
    bar.set_style(ProgressStyle::default_bar()
       .template("LOADING PPM: {percent}% {bar:40.cyan/blue} {pos:>7}/{len:7}"));

    let mut ppm = PPM::new(image_width, image_height);

    // Camera
    let camera = Camera::new(2.0, aspect_ratio * 2.0, 1.0, Vector3::new(0.0, 0.0, 0.0));

    let lower_left_corner = camera.position
                          - camera.horizontal/2.0 
                          - camera.vertical/2.0 
                          - Vector3::new(0.0, 0.0, camera.focal_length);
    
    for (j, i) in iproduct!((0..image_height).rev(), 0..image_width) {
        bar.inc(1);
        
        let u = i as f64 / (image_width - 1) as f64;
        let v = j as f64 / (image_height - 1) as f64;
        let r = Ray::new(camera.position, lower_left_corner +
                                 u*camera.horizontal + v*camera.vertical
                                 - camera.position);
        
        ppm.set(i, image_height - j - 1, ray_color(r));
    }
    bar.finish();

    println!("{:?}", ppm);
}