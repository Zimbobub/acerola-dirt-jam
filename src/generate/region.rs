use crate::generate::Pos;







pub struct RegionCoords {
    pub x: i64,
    pub y: i64,
}



/// Lazy means only centroids have been generated
pub enum RegionState {
    LazyGenerated,
    FullyGenerated
}


/// A region is a 64x64 vertex area
/// The chunks in a region can extend out into its neighbors
/// A region has 5 voronoi centroids
pub struct Region {
    coords: RegionCoords,
    state: RegionState,
    centroids: [Pos; 5]
}
