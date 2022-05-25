use super::vector3::Vector3;

pub struct Camera {
    pub viewport_height: f64, 
    pub viewport_width: f64,
    pub focal_length: f64,
    
    pub position: Vector3,

    pub horizontal: Vector3,
    pub vertical: Vector3,
}

impl Camera {
    pub fn new(viewport_height: f64, viewport_width: f64, focal_length: f64, position: Vector3) -> Self {
        Self {
            viewport_height,
            viewport_width,
            focal_length,
            position,
            horizontal: Vector3::new(viewport_width, 0.0, 0.0),
            vertical: Vector3::new(0.0, viewport_height, 0.0),
        } 
    }
}