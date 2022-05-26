use crate::math::ray::Ray;
use crate::math::vector3::Vector3;

pub struct Sphere {
    position: Vector3,
    radius: f64,
}

impl Sphere {
    pub fn new(position: Vector3, radius: f64) -> Self {
        Self { position, radius }
    }

    pub fn hit(&self, ray: &Ray) -> bool {
        let offset_center = ray.origin - self.position;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * offset_center.dot(&ray.direction);
        let c = offset_center.dot(&offset_center) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        discriminant > 0.0
    }
}
