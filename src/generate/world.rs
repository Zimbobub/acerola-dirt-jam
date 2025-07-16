use crate::generate::region::{Region, RegionCoords};







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

    pub fn generate_region(&mut self, coords: RegionCoords) {

    }
}

