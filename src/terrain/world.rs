use crate::terrain::chunk::Chunk;





pub struct World<'world> {
    pub chunks: Vec<Chunk<'world>>
}


impl World<'_> {
    pub fn new() -> Self {
        return Self {
            chunks: Vec::new()
        };
    }
}