use std::collections::HashMap;

use spade::{FloatTriangulation, Point2, Triangulation};

use crate::terrain::{chunk::{Chunk, ChunkId}, world::World, Pos};





/// Creates the mesh from a `World`
#[derive(Debug)]
pub struct SampledWorld {
    pub chunks: HashMap<ChunkId, Chunk>,
}


impl SampledWorld {
    pub fn init(world_save: World, player_pos: Pos, radius: f64) -> Self {
        let mut chunks: HashMap<ChunkId, Chunk> = HashMap::new();

        // for edge in world_save.triangulation.get_edges_in_circle(player_pos.into(), radius) {
        for face in world_save.triangulation.inner_faces() {
            // if all verticies outside render radius
            if face.positions().iter().all(|vertex| !point_in_circle(Pos::from(*vertex), player_pos, radius)) {
                continue;
            }

            let mut is_border = false;

            let neighbors: [Option<ChunkId>; 3] = face.adjacent_edges().map(|e| {
                let neighbor_face = e.face();
                // outer face of the triangluation, ignore
                if neighbor_face.is_outer() {
                    is_border = true;
                    return None;
                }

                let neighbor_face = neighbor_face.as_inner().unwrap();



                // check if neighbor face is outside the circle at all
                // if any vertex not in circle
                if neighbor_face.positions().iter().any(|vertex| 
                    !point_in_circle(vertex.clone().into(), player_pos, radius)
                ) {
                    is_border = true;
                    return None;
                }
                return Some(neighbor_face.index());
            });

            let points = face.positions();

            chunks.insert(face.index(), Chunk::new(
                face.index(),
                [points[0].into(), points[1].into(), points[2].into()],
                neighbors,
                is_border
            ));
        }

        return Self {
            chunks: chunks
        };
    }
}




fn point_in_circle(point: Pos, circle_centre: Pos, radius: f64) -> bool {
    let dx = point.x - circle_centre.x;
    let dy = point.y - circle_centre.y;
    return dx * dx + dy * dy <= radius * radius;
}