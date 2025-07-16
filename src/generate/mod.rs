




pub mod chunk;
pub mod region;
pub mod world;


#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}


impl Pos {
    pub fn new(x: i64, y: i64) -> Self {
        return Self { x: x, y: y };
    }
}



pub struct Rect {
    pub x1: i64,
    pub y1: i64,
    pub x2: i64,
    pub y2: i64
}

impl Rect {
    pub fn new(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
        return Self { x1: x1, y1: y1, x2: x2, y2: y2 };
    }

    pub fn from_pos(pos1: Pos, pos2: Pos) -> Self {
        return Rect::new(pos1.x, pos1.y, pos2.x, pos2.y);
    }

    pub fn width(&self) -> i64 {
        return (self.x1 - self.x2).abs();
    }

    pub fn height(&self) -> i64 {
        return (self.y1 - self.y2).abs();
    }

    pub fn top_left(&self) -> Pos {
        return Pos::new(i64::min(self.x1, self.x2), i64::min(self.y1, self.y2))
    }

    pub fn top_right(&self) -> Pos {
        return Pos::new(i64::max(self.x1, self.x2), i64::min(self.y1, self.y2))
    }

    pub fn bottom_left(&self) -> Pos {
        return Pos::new(i64::min(self.x1, self.x2), i64::max(self.y1, self.y2))
    }

    pub fn bottom_right(&self) -> Pos {
        return Pos::new(i64::max(self.x1, self.x2), i64::max(self.y1, self.y2))
    }
}