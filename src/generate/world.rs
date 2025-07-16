use image::RgbImage;

use crate::generate::{region::Region, Pos, Rect};







/// Used to store all terrain data
/// Stores centroids of half generated regions
pub struct World {
    regions: Vec<Region>
}

impl World {
    pub fn new() -> Self {
        return Self { regions: Vec::new() };
    }


    /// Generation of initial region works diffrently as it has no neighbors to base off of
    pub fn generate_initial_region(&mut self) {

    }

    pub fn generate_region(&mut self, coords: Pos) {

    }
}


/// Export data to images to debug
impl World {
    pub fn get_world_bounds(&self) -> Rect {
        let mut min_x: i64 = 0;
        let mut max_x: i64 = 0;
        let mut min_y: i64 = 0;
        let mut max_y: i64 = 0;

        for region in self.regions.iter() {
            let bounds = region.get_bounds();
            let top_left = bounds.top_left();
            let bottom_right = bounds.bottom_right();

            if top_left.x < min_x { min_x = top_left.x }
            if top_left.y < min_y { min_y = top_left.y }
            if bottom_right.x > max_x { max_x = bottom_right.x }
            if bottom_right.y > max_y { max_y = bottom_right.y }
        }

        return Rect::new(min_x, min_y, max_x, max_y);
    }


    pub fn export_centroids(&self) {
        let bounds = self.get_world_bounds();
        let mut img = RgbImage::new(bounds.width() as u32, bounds.height() as u32);

        for region in self.regions.iter() {
            let region_bounds = region.get_bounds();
        }
    }
}

