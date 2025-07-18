
use crate::terrain::Pos;



pub type ChunkId = usize;

#[derive(Debug)]
pub struct Chunk {
    id: ChunkId,
    verticies: [Pos; 3],
    /// Triangles can only have max of 3 neighbors
    neighbors: [Option<ChunkId>; 3],
    /// True if this chunk is on the outer boundary of the rendered world
    is_border: bool,
}


impl Chunk {
    pub fn new(id: ChunkId, verticies: [Pos; 3], neighbors: [Option<ChunkId>; 3], is_border: bool) -> Self {
        return Self {
            id: id,
            verticies: verticies,
            neighbors: neighbors,
            is_border: is_border
        };
    }
}