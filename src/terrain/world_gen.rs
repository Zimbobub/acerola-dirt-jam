use spade::{DelaunayTriangulation, Point2, Triangulation};

use crate::terrain::{chunk::Chunk, world_save::WorldSave, Pos};





/// Creates the mesh from a `WorldSave`
pub struct WorldGen<'world> {
    pub centroids: Vec<Pos>,
    pub triangulation: DelaunayTriangulation<Point2<f64>>,
    pub chunks: Vec<Chunk<'world>>,
}


impl WorldGen<'_> {
    pub fn init(world_save: WorldSave, player_pos: Pos, radius: f64) -> Self {
        let mut centroids: Vec<Pos> = Vec::new();

        for region in world_save.regions.values() {
            for pos in region {
                let taxicab_distance = (pos.x - player_pos.x).abs() + (pos.y - player_pos.y).abs();
                if taxicab_distance <= radius {
                    centroids.push(*pos);
                };
            }
        }

        return Self {
            centroids: centroids,
            triangulation: DelaunayTriangulation::new(),
            chunks: Vec::new()
        };
    }
}


