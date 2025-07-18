
use std::collections::HashMap;

use spade::{handles::FixedVertexHandle, DelaunayTriangulation, Triangulation};

use crate::terrain::{region::{RegionPos, REGION_CHUNKS, REGION_SIZE}, Pos};




/// Base data used to generate the world
/// If there were save files, this is all that would need to be stored
/// `WorldGen` handles triangulation and actual mesh generation
#[derive(Debug)]
pub struct World {
    pub regions: HashMap<RegionPos, Vec<Pos>>,
    pub triangulation: DelaunayTriangulation<Pos>,
}


impl World {
    pub fn new() -> Self {
        return Self {
            regions: HashMap::new(),
            triangulation: DelaunayTriangulation::new()
        };
    }


    pub fn generate_region(&mut self, region_pos: RegionPos) {
        // skip if already generated
        if self.regions.contains_key(&region_pos) { return; }
        
        let real_pos: Pos = region_pos.into();
        let centroids: Vec<Pos> = (0..rand::random_range(REGION_CHUNKS)).map(|_| {
            return Pos::new(
                rand::random_range(real_pos.x..real_pos.x+REGION_SIZE),
                rand::random_range(real_pos.y..real_pos.y+REGION_SIZE),
            );
        }).collect();

        // add all centroids to triangulation
        let mut prev: Option<FixedVertexHandle> = None;
        for centroid in centroids.iter() {
            match prev {
                // tell `spade` to first try previous point, as it is nearby
                Some(prev_vertex) => {
                    prev = self.triangulation.insert_with_hint(
                        *centroid,
                        prev_vertex
                    ).ok();
                },
                None => {
                    prev = self.triangulation.insert(*centroid).ok();
                }
            };
        }

        self.regions.insert(region_pos, centroids);
    }
}
