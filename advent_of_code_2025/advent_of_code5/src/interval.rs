

pub struct Interval {
    pub start: u64,
    pub end: u64,
}



impl Interval {
    pub fn new(start: u64, end: u64) -> Self {
        Self{start: start, end: end}
    }

    pub fn clone(&self) -> Self {
        Self{start: self.start, end: self.end}
    }
}