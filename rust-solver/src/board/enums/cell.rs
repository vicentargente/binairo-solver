use serde::{Deserialize, Serialize};

use crate::board::coords::Coords;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cell {
    Empty,
    Black,
    White
}

#[allow(dead_code)]
impl Cell {
    pub fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }

    pub fn is_black(&self) -> bool {
        matches!(self, Cell::Black)
    }

    pub fn is_white(&self) -> bool {
        matches!(self, Cell::White)
    }

    pub fn get_next_state(&self) -> Cell {
        match self {
            Cell::Empty => Cell::Black,
            Cell::Black => Cell::White,
            Cell::White => Cell::Empty
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum FromCellCoords {
    Up(Coords),
    // Down(Coords),
    Left(Coords),
    // Right(Coords)
}


#[derive(Debug, Clone, Deserialize)]
pub struct CellCoords {
    pub(crate) coords: Coords,
    pub(crate) cell: Cell
}

#[allow(dead_code)]
impl CellCoords {
    pub fn coords(&self) -> &Coords {
        &self.coords
    }

    pub fn cell(&self) -> &Cell {
        &self.cell
    }
}

