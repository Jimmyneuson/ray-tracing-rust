use super::vector3::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_at() {
        let r = Ray::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
        );

        assert_eq!(
            r.at(0.5),
            Vector3::new(0.5, 0.5, 0.5)
        );
    }
}
