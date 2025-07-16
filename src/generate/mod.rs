
pub struct Pos {
    pub x: f64,
    pub y: f64,
}



/// A chunk is a voronoi polygon within a region
pub struct Chunk {

}




pub struct RegionCoords {
    pub x: i64,
    pub y: i64,
}

/// A region is a 64x64 vertex area
/// The chunks in a region can extend out into its neighbors
/// A region has 5 voronoi centroids
pub struct Region {
    coords: RegionCoords
}


/// Used to store all terrain data
/// Stores centroids of half generated regions
pub struct World {

}

impl World {
    pub fn generate_region(&mut self, coords: RegionCoords) {

    }
}

