use spade::{DelaunayTriangulation, Point2, Triangulation};

use crate::terrain::{chunk::Chunk, Pos, RegionPos, REGION_CHUNKS, REGION_SIZE};





pub struct World<'world> {
    pub chunks: Vec<Chunk<'world>>,
    pub triangulation: DelaunayTriangulation<Point2<f64>>
}


impl World<'_> {
    pub fn new() -> Self {
        return Self {
            chunks: Vec::new(),
            triangulation: DelaunayTriangulation::new()
        };
    }


    pub fn generate_region(&mut self, region_pos: RegionPos) {
        let real_pos: Pos = region_pos.into();
        let centroids: Vec<Pos> = (0..rand::random_range(REGION_CHUNKS)).map(|i| {
            return Pos::new(
                rand::random_range(real_pos.x..real_pos.x+REGION_SIZE),
                rand::random_range(real_pos.y..real_pos.y+REGION_SIZE),
            );
        }).collect();

        dbg!(centroids);
    }
}