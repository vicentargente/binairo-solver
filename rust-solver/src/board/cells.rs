use crate::board::{coords::Coords, enums::cell::{Cell, CellCoords}};

pub struct Cells {
    size: u32,
    grid: Vec<Cell>
}

impl Cells {
    pub fn new(size: u32) -> Self {
        Cells {
            size,
            grid: vec![Cell::Empty; size as usize * size as usize]
        }
    }

    pub fn from_cells(size: u32, cells: Vec<CellCoords>) -> Result<Self, &'static str> {
        let mut cells_grid = Cells::new(size);
        for CellCoords { coords, cell } in cells {
            cells_grid.set_cell(&coords, cell)?;
        }

        Ok(cells_grid)
    }

    pub fn get_cell(&self, coords: &Coords) -> Result<Cell, &'static str> {
        let index = coords.to_index(self.size);
        if index >= self.grid.len() {
            return Err("Index out of bounds");
        }

        Ok(self.grid[index])
    }

    pub fn set_cell(&mut self, coords: &Coords, cell: Cell) -> Result<(), &'static str> {
        let index = coords.to_index(self.size);
        if index >= self.grid.len() {
            return Err("Index out of bounds");
        }

        self.grid[index] = cell;

        Ok(())
    }

    pub fn cells(&self) -> &Vec<Cell> {
        &self.grid
    }
}
