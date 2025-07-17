


pub struct Chunk<'world> {
    neighbors: Vec<&'world Chunk<'world>>
}