use crate::math::vector3::Vector3;

pub struct PPM {
    columns: usize,
    rows: usize,
    pub pixels: Vec<RGBTriplet>,
}

impl PPM {
    pub fn new(columns: u64, rows: u64) -> Self {
        let columns = columns as usize;
        let rows = rows as usize;

        Self {
            columns,
            rows,
            pixels: vec![RGBTriplet::default(); columns * rows],
        }
    }

    pub fn set(
        &mut self,
        column: u64,
        row: u64,
        new: RGBTriplet,
    ) -> RGBTriplet {
        let column = column as usize;
        let row = row as usize;

        self.pixels[self.columns * row + column] = new;
        new
    }

    pub fn get(&self, column: usize, row: usize) -> Option<&RGBTriplet> {
        self.pixels.get(self.columns * row + column)
    }
}

impl std::fmt::Debug for PPM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pixel_format: String = self
            .pixels
            .iter()
            .map(|pixel| pixel.to_string() + "\n")
            .collect();

        write!(
            f,
            "P3\n{} {}\n255\n{}",
            self.columns, self.rows, pixel_format
        )
    }
}

impl ToString for PPM {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Copy, Clone)]
pub struct RGBTriplet {
    r: u8,
    g: u8,
    b: u8,
}

impl RGBTriplet {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_vector3(vector: Vector3) -> Self {
        Self {
            r: vector.x as u8,
            g: vector.y as u8,
            b: vector.z as u8,
        }
    }
}

impl ToString for RGBTriplet {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.r, self.g, self.b)
    }
}

impl Default for RGBTriplet {
    fn default() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
        }
    }
}

impl std::fmt::Debug for RGBTriplet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.r, self.g, self.b
        )
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn gradient() {
        let mut ppm = PPM::new(256, 256);
        let cmp = read_to_string("./tests/gradient.ppm").unwrap();

        for j in (0..256).rev() {
            for i in 0..256 {
                ppm.set(
                    i,
                    256 - j - 1,
                    RGBTriplet::new(
                        ((i as f64 / 255.0) * 255.999) as u8,
                        ((j as f64 / 255.0) * 255.999) as u8,
                        (0.25 * 255.999) as u8,
                    ),
                );
            }
        }

        assert_eq!(ppm.to_string(), cmp);
    }

    #[test]
    fn three_by_two() {
        let mut ppm = PPM::new(3, 2);
        let cmp = read_to_string("./tests/three_by_two.ppm").unwrap();

        ppm.set(0, 0, RGBTriplet::new(255, 0, 0));
        ppm.set(1, 0, RGBTriplet::new(0, 255, 0));
        ppm.set(2, 0, RGBTriplet::new(0, 0, 255));
        ppm.set(0, 1, RGBTriplet::new(255, 255, 0));
        ppm.set(
            1,
            1,
            RGBTriplet::new(255, 255, 255),
        );
        ppm.set(2, 1, RGBTriplet::new(0, 0, 0));

        assert_eq!(ppm.to_string(), cmp);
    }
}
