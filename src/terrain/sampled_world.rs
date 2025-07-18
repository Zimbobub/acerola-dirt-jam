use std::collections::HashMap;

use spade::{FloatTriangulation, Point2};

use crate::terrain::{chunk::{Chunk, ChunkId}, world::World, Pos};





/// Creates the mesh from a `World`
#[derive(Debug)]
pub struct SampledWorld {
    pub chunks: HashMap<ChunkId, Chunk>,
}


impl SampledWorld {
    pub fn init(world_save: World, player_pos: Pos, radius: f64) -> Self {
        // let mut centroids: Vec<Pos> = Vec::new();

        // for region in world_save.regions.values() {
        //     for pos in region {
        //         let taxicab_distance = (pos.x - player_pos.x).abs() + (pos.y - player_pos.y).abs();
        //         if taxicab_distance <= radius {
        //             centroids.push(*pos);
        //         };
        //     }
        // }


        let mut chunks: HashMap<ChunkId, Chunk> = HashMap::new();
        
        for edge in world_save.triangulation.get_edges_in_circle(Point2::new(player_pos.x, player_pos.y), radius) {
            match edge.as_directed().face().as_inner() {
                Some(face) => {
                    let mut is_border = false;

                    let neighbors: [Option<ChunkId>; 3] = face.adjacent_edges().map(|e| {
                        match e.rev().face().as_inner() {
                            // outer face of the triangluation, ignore
                            None => {
                                is_border = true;
                                return None;
                            }
                            Some(neighbor_face) => {
                                // check if neighbor face is outside the circle at all
                                // if any vertex not in circle
                                if neighbor_face.positions().iter().any(|vertex| 
                                    !point_in_circle(vertex.clone().into(), player_pos, radius)
                                ) {
                                    is_border = true;
                                    return None;
                                }
                                return Some(neighbor_face.index());
                            }
                        }
                    });

                    let points = face.positions();

                    chunks.insert(face.index(), Chunk::new(
                        face.index(),
                        [points[0].into(), points[1].into(), points[2].into()],
                        neighbors,
                        is_border
                    ));
                },
                None => continue
            };
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