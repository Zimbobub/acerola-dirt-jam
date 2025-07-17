
use crate::terrain::{Pos, RegionPos, REGION_CHUNKS, REGION_SIZE};





pub struct WorldSave {
    pub centroids: Vec<Pos>,
}


impl WorldSave {
    pub fn new() -> Self {
        return Self {
            centroids: Vec::new(),
        };
    }


    pub fn generate_region(&mut self, region_pos: RegionPos) {
        let real_pos: Pos = region_pos.into();
        for _ in 0..rand::random_range(REGION_CHUNKS) {
            let pos = Pos::new(
                rand::random_range(real_pos.x..real_pos.x+REGION_SIZE),
                rand::random_range(real_pos.y..real_pos.y+REGION_SIZE),
            );

            self.centroids.push(pos);
        }
    }
}