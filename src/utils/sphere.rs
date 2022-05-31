use super::hittable::*;
use crate::math::ray::Ray;
use crate::math::vector3::Vector3;
use std::ops::RangeInclusive;

pub struct Sphere {
    pub position: Vector3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(position: Vector3, radius: f64) -> Self {
        Self { position, radius }
    }

    // pub fn hit(&self, ray: &Ray) -> f64 {
    //     let offset_center = ray.origin - self.position;
    //     let a = ray.direction.magnitude().powi(2);
    //     let half_b = offset_center.dot(ray.direction);
    //     let c = offset_center.magnitude().powi(2) - self.radius.powi(2);
    //     let discriminant = half_b.powi(2) - a * c;

    //     if discriminant < 0.0 {
    //         -1.0
    //     } else {
    //         (-half_b - discriminant.sqrt()) / a
    //     }
    // }
}

impl Hittable for Sphere {
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
