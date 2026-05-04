
#[derive(Debug, Copy, Clone)]
pub struct JunctionBox {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl JunctionBox {
    pub fn new(x: usize, y: usize, z: usize) -> JunctionBox {
        JunctionBox { x, y, z }
    }


}