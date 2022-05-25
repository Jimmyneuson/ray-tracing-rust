use super::vector3::Vector3;

pub struct Camera {
    pub viewport_height: f64, 
    pub viewport_width: f64,
    pub focal_length: f64,
    
    pub position: Vector3,

    pub horizontal: Vector3,
    pub vertical: Vector3,
}

impl Camera {
    pub fn new(viewport_height: f64, viewport_width: f64, focal_length: f64, position: Vector3) -> Self {
        Self {
            viewport_height,
            viewport_width,
            focal_length,
            position,
            horizontal: Vector3::new(viewport_width, 0.0, 0.0),
            vertical: Vector3::new(0.0, viewport_height, 0.0),
        } 
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::ppm::RGBTriplet;

    use super::*;
    use super::super::ppm::PPM;
    use super::super::ray::Ray;
    use itertools::iproduct;
    
    #[test]
    fn sky() {
        let aspect_ratio = 16.0 / 9.0;           
        let image_width = 400;
        let image_height = (image_width as f64 / aspect_ratio) as u64;
        
        let origin = Vector3::new(0.0, 0.0, 0.0);
        
        let mut ppm = PPM::new(image_width, image_height);
        
        let camera = Camera::new(2.0, aspect_ratio * 2.0, 1.0, origin);
        
        let lower_left_corner = camera.position
                              - camera.horizontal / 2.0
                              - camera.vertical / 2.0
                              - Vector3::new(0.0, 0.0, camera.focal_length);
        
        for (j, i) in iproduct!((0..image_height).rev(), 0..image_height) {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let mut r = Ray::new(camera.position, lower_left_corner
                                                + u * camera.horizontal
                                                + v * camera.vertical
                                                - camera.position);
            
            let unit_direction = r.direction.unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            let v = Vector3::lerp(&Vector3::new(1.0, 1.0, 1.0),
                                  &Vector3::new(0.5, 0.7, 1.0), t).color();
            ppm.set(i, image_height - j - 1,
                    RGBTriplet::new(v.x as u8, v.y as u8, v.z as u8));
        }
    }
}