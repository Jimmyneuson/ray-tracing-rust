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
        let a = ray.direction.magnitude().powi(2);
        let half_b = offset_center.dot(ray.direction);
        let c = offset_center.magnitude().powi(2) - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }
}
