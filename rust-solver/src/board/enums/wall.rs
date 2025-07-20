use serde::Deserialize;

use crate::board::enums::cell::{Cell, FromCellCoords};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Wall {
    Normal,
    Equals,
    NotEquals
}

impl Wall {
    pub fn check_cells(&self, cell1: &Cell, cell2: &Cell) -> bool {
        match self {
            Wall::Normal => true,
            Wall::Equals => cell1 == cell2,
            Wall::NotEquals => cell1 != cell2
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WallCoords {
    pub(crate) coords: FromCellCoords,
    pub(crate) wall: Wall
}

#[allow(dead_code)]
impl WallCoords {
    pub fn coords(&self) -> &FromCellCoords {
        &self.coords
    }

    pub fn wall(&self) -> &Wall {
        &self.wall
    }
}

