use std::ops;

#[derive(PartialEq, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    
    pub fn dot(&self, other: Vector3) -> f64 {
          self.x * other.x
        + self.y * other.y
        + self.z * other.z
    }
    
    pub fn cross(&mut self, other: Vector3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    
    pub fn unit(&mut self) -> Self {
        *self / self.magnitude()
    }

    pub fn lerp(left: &Vector3, right: &Vector3, t: f64) -> Self {
        (1.0 - t) * *left + t * *right
    }
    
    pub fn color(&self) -> Self {
        Self {
            x: 255.999 * self.x,
            y: 255.999 * self.y,
            z: 255.999 * self.z,
        } 
    }
}

impl ops::Neg for Vector3 {
    type Output = Self; 
    
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Add for Vector3 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        } 
    }
}

impl ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Self;
    
    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,         
            y: self.y * scalar,
            z: self.z * scalar,
        }  
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, vector: Vector3) -> Vector3 {
        Vector3::new(
            vector.x * self,
            vector.y * self,
            vector.z * self,
        )
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Self;
    
    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,         
            y: self.y / scalar,
            z: self.z / scalar,
        }  
    }
}

impl std::fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*; 
    
    #[test]
    fn negate_vector() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(-v, Vector3::new(-1.0, -2.0, -3.0));
    }
    
    #[test]
    fn add_vector() {
        let v1 = Vector3::new(1.0, 1.0, 1.0);
        let v2 = Vector3::new(2.0, 2.0, 2.0);
        assert_eq!(v1 + v2, Vector3::new(3.0, 3.0, 3.0));
    }
    
    #[test]
    fn scalar_mul_vector() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let s = 2.0;
        assert_eq!(v * s, Vector3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector_mul_scalar() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let s = 2.0;
        assert_eq!(s * v, Vector3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn div_vector() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let s = 2.0;
        assert_eq!(v / s, Vector3::new(0.5, 1.0, 1.5));
    }
    
    #[test]
    fn magnitude_vector() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 14f64.sqrt());
    }
    
    #[test]
    fn dot_vector() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.dot(v2), 14.0);
    }
    
    #[test]
    fn cross_vector() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(1.0, 4.0, 1.0);
        assert_eq!(v1.cross(v2), Vector3::new(-10.0, 2.0, 2.0));
    }
    
    #[test]
    fn cross_vector_with_itself() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.cross(v1), Vector3::new(0.0, 0.0, 0.0));
    }
    
    #[test]
    fn unit_vector() {
        let mut v = Vector3::new(1.0, 2.0, 3.0); 
        assert_eq!(v.unit(), Vector3::new(
            1.0/14f64.sqrt(), 
            (2.0/7.0f64).sqrt(), 
            3.0/14f64.sqrt()));
    }
    
    #[test]
    fn unit_vector_length_is_one() {
        let mut v = Vector3::new(1.0, 2.0, 3.0); 
        assert_eq!(v.unit().magnitude(), 1.0);
    }

    #[test]
    fn vector_lerp_half() {
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(1.0, 1.0, 1.0);
        assert_eq!(Vector3::lerp(&v1, &v2, 0.5), 
                   Vector3::new(0.5, 0.5, 0.5));
    }

    #[test]
    fn vector_lerp_three_quarters() {
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(1.0, 1.0, 1.0);
        assert_eq!(Vector3::lerp(&v1, &v2, 0.75), 
                   Vector3::new(0.75, 0.75, 0.75));
    }
}
