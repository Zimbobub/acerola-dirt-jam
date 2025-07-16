use std::collections::HashMap;

use image::{Rgb, RgbImage};

use crate::generate::{region::{Region, RegionState}, Pos, Rect};







/// Used to store all terrain data
/// Stores centroids of half generated regions
pub struct World {
    regions: HashMap<Pos, Region>
}

impl World {
    pub fn new() -> Self {
        return Self { regions: HashMap::new() };
    }


    /// Generation of initial region works diffrently as it has no neighbors to base off of
    pub fn generate_initial_region(&mut self) {
        self.generate_region(Pos::new(0, 0));
    }

    pub fn generate_region(&mut self, coords: Pos) -> Option<()> {
        if self.regions.contains_key(&coords) { return None }

        let mut region = Region::generate(coords);
        // then lazy generate neighbors
        // then fully generate this region

        self.regions.insert(coords, region);

        return Some(());
    }
}


/// Export data to images to debug
impl World {
    pub fn get_world_bounds(&self) -> Rect {
        let mut min_x: i64 = 0;
        let mut max_x: i64 = 0;
        let mut min_y: i64 = 0;
        let mut max_y: i64 = 0;

        for region in self.regions.values() {
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
        let world_bounds = self.get_world_bounds();
        let mut img = RgbImage::new(world_bounds.width() as u32, world_bounds.height() as u32);

        for region in self.regions.values() {
            let img_region_bounds = region.get_bounds().translate(world_bounds.top_left());

            // grey background based on region state
            for x in img_region_bounds.top_left().x..img_region_bounds.bottom_right().x {
                for y in img_region_bounds.top_left().y..img_region_bounds.bottom_right().y {
                    img.put_pixel(x as u32, y as u32, match region.state {
                        RegionState::LazyGenerated => Rgb([50, 50, 50]),
                        RegionState::FullyGenerated => Rgb([100, 100, 100]),
                    });
                }
            }

            // centroids
            for centroid in region.centroids {
                img.put_pixel(
                    (img_region_bounds.top_left().x + centroid.x) as u32,
                    (img_region_bounds.top_left().y + centroid.y) as u32,
                    Rgb([255, 0, 0])
                );
            }
        }

        img.save("./debug/centroids.png").unwrap();
    }
}

