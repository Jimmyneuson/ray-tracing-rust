use crate::math::ray::Ray;
use crate::math::vector3::Vector3;
use std::ops::RangeInclusive;

/// Describes where a hit happens and it's normal.
pub struct Hit {
    pub position: Vector3,
    pub normal: Vector3,
    /// Interval for the `Ray` object.
    pub t: f64,
}

impl Hit {
    /// Set the face normal to always point outwards.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

/// Returns a `Hit` object if a certain ray hits it.
pub trait Hittable {
    fn hit(&self, ray: &Ray, valid_range: RangeInclusive<f64>) -> Option<Hit>;
}
