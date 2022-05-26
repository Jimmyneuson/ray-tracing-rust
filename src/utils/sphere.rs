use crate::math::ray::Ray;
use crate::math::vector3::Vector3;

pub struct Sphere {
    pub position: Vector3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(position: Vector3, radius: f64) -> Self {
        Self { position, radius }
    }

    pub fn hit(&self, ray: &Ray) -> f64 {
        let offset_center = ray.origin - self.position;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * offset_center.dot(&ray.direction);
        let c = offset_center.dot(&offset_center) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (-b - discriminant.sqrt()) / (2.0 * a)
        }
    }
}
