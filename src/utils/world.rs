use std::ops::RangeInclusive;

use super::hittable::*;
use crate::math::ray::Ray;

#[derive(Default)]
pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for World {
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
