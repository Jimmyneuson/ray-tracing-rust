use super::hittable::*;
use crate::math::ray::Ray;
use crate::math::vector3::Vector3;
use std::ops::RangeInclusive;

/// Holds information for a sphere.
pub struct Sphere {
    pub position: Vector3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(position: Vector3, radius: f64) -> Self {
        Self { position, radius }
    }
}

impl Hittable for Sphere {
    /// Performs a ray-sphere intersection and returns the `Hit` object.
    fn hit(&self, ray: &Ray, valid_range: RangeInclusive<f64>) -> Option<Hit> {
        let oc = ray.origin - self.position;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - (self.radius * self.radius);

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let mut root = (-half_b - discriminant.sqrt()) / a;
        if !valid_range.contains(&root) {
            root = (-half_b + discriminant.sqrt()) / a;
            if !valid_range.contains(&root) {
                return None;
            }
        }

        let pos = ray.at(root);
        let outward_normal = (pos - self.position) / self.radius;
        let mut hit = Hit {
            position: pos,
            normal: outward_normal,
            t: root,
        };

        hit.set_face_normal(ray, outward_normal);

        Some(hit)
    }
}
