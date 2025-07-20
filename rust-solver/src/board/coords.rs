use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Coords {
    pub x: u32,
    pub y: u32
}

#[allow(dead_code)]
impl Coords {
    pub fn new(x: u32, y: u32) -> Self {
        Coords { x, y }
    }

    pub fn from_tuple(coords: (u32, u32)) -> Self {
        Coords { x: coords.0, y: coords.1 }
    }

    pub fn to_tuple(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    pub fn from_index(index: usize, size: usize) -> Self {
        let x = (index % size) as u32;
        let y = (index / size) as u32;
        Coords { x, y }
    }

    pub fn to_index(&self, size: u32) -> usize {
        (self.y as usize) * (size as usize) + (self.x as usize)
    }
}
