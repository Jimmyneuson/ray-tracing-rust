use crate::math::ray::Ray;
use crate::math::vector3::Vector3;
use std::ops::RangeInclusive;

pub struct Hit {
    pub position: Vector3,
    pub normal: Vector3,
    pub t: f64,
}

impl Hit {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, valid_range: RangeInclusive<f64>) -> Option<Hit>;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, valid_range: RangeInclusive<f64>) -> Option<Hit> {
        let mut closest = *valid_range.end();
        let mut hit_anything = None;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, *valid_range.start()..=closest)
            {
                closest = hit.t;
                hit_anything = Some(hit);
            }
        }

        hit_anything
    }
}
