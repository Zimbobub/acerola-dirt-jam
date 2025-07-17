use crate::generate::Pos;




#[derive(Debug, Clone)]
/// A chunk is a voronoi polygon within a region
pub struct Chunk {
    pub id: (Pos, usize)
}

