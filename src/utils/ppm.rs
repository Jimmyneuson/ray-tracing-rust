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
    
    pub fn set(&mut self, column: u64, row: u64, new: RGBTriplet) -> RGBTriplet {
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
        let pixel_format: String = self.pixels
                                       .iter()
                                       .map(|pixel| pixel.to_string() + "\n")
                                       .collect();
        write!(f, "P3\n{} {}\n255\n{}", self.columns, self.rows, pixel_format)
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
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}
