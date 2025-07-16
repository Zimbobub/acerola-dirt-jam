use std::collections::HashMap;

use image::{GenericImage, Rgb, RgbImage};

use crate::generate::{region::Region, Pos, Rect};







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
        println!("Generating region ({}, {})", coords.x, coords.y);

        // check if already generated
        match self.regions.get(&coords) {
            None | Some(Region::LazyGenerated(_)) => {},
            Some(Region::FullyGenerated(_)) => {
                println!("    Error: region already generated");
                return None;
            }
        }

        if coords.x.trailing_zeros() < 6 || coords.y.trailing_zeros() < 6 {
            println!("Error generating region: coordinates must be multiple of 64");
            return None;
        }

        // lazy generate the region
        let region = Region::init(coords);

        // then lazy generate neighbors
        for neighbor_coords in region.get_neighbor_coords() {
            self.lazy_generate_region(neighbor_coords);
        }

        // then fully generate this region
        self.regions.insert(coords, region.fully_generate()?);


        println!("    Success");
        return Some(());
    }


    fn lazy_generate_region(&mut self, coords: Pos) {
        println!("Lazy generating region ({}, {})", coords.x, coords.y);

        if self.regions.contains_key(&coords) {
            println!("    Error: region already generated");
            return;
        }

        self.regions.insert(coords, Region::init(coords));
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
        // create images for each region then stitch them together
        let region_imgs: Vec<(Pos, image::ImageBuffer<Rgb<u8>, Vec<u8>>)> = self.regions.values().map(|region| {
            println!("Creating region image for {}, {}", region.coords().x, region.coords().y);

            let mut region_img: image::ImageBuffer<Rgb<u8>, Vec<u8>> = RgbImage::new(64, 64);

            // fill background
            let background: Rgb<u8> = match region {
                Region::LazyGenerated(_) => Rgb([50, 50, 50]),
                Region::FullyGenerated(_) => Rgb([100, 100, 100]),
            };

            for pixel in region_img.pixels_mut() {
                *pixel = background;
            }

            // plot centroids
            // centroid coords are local coordinates (0..=63)
            for centroid in region.centroids() {
                region_img.put_pixel((centroid.x - region.coords().x) as u32, (centroid.y - region.coords().y) as u32, Rgb([255, 0, 0]));
            }

            return (region.coords(), region_img);
        }).collect();


        // stitch all images together
        let world_bounds = self.get_world_bounds();
        let mut img = RgbImage::new(world_bounds.width() as u32, world_bounds.height() as u32);
        println!("Image is {}x{}", world_bounds.width() as u32, world_bounds.height() as u32);


        for (pos, region_img) in region_imgs {
            let (x, y) = to_pixel_coords(world_bounds.top_left(), pos);
            img.copy_from(&region_img, x, y).unwrap();
        }




        img.save("./debug/centroids.png").unwrap();
    }
}



fn to_pixel_coords(top_left: Pos, pos: Pos) -> (u32, u32) {
    return ((pos.x - top_left.x) as u32, (pos.y - top_left.y) as u32);
}