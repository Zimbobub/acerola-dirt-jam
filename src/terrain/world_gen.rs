use spade::{DelaunayTriangulation, Point2};

use crate::terrain::{chunk::Chunk, Pos};






pub struct WorldGen<'world> {
    pub centroids: Vec<Pos>,
    pub triangulation: DelaunayTriangulation<Point2<f64>>,
    pub chunks: Vec<Chunk<'world>>,
}



