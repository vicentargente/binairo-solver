use crate::board::{cells::Cells, coords::Coords, enums::{cell::{Cell, CellCoords, FromCellCoords}, wall::WallCoords}, walls::Walls};

mod cells;
mod walls;
pub mod coords;
pub mod enums;

pub struct Board {
    size: u32,
    cells: Cells,
    walls: Walls
}

#[allow(dead_code)]
impl Board {
    pub fn new(size: u32) -> Self {
        Board {
            size,
            cells: Cells::new(size),
            walls: Walls::new(size)
        }
    }

    pub fn from_cells_and_walls(size: u32, cells: Vec<CellCoords>, walls: Vec<WallCoords>) -> Result<Self, &'static str> {
        let cells = Cells::from_cells(size, cells)?;
        let walls = Walls::from_walls(size, walls)?;

        Ok(Board {
            size,
            cells,
            walls
        })
    }

    pub fn get_cell(&self, coords: &Coords) -> Result<Cell, &'static str> {
        self.cells.get_cell(coords)
    }

    pub fn set_cell(&mut self, coords: &Coords, cell: Cell) -> Result<(), &'static str> {
        self.cells.set_cell(coords, cell)
    }

    pub fn get_empty_cells(&self) -> Result<Vec<Coords>, &'static str> {
        let mut empty_cells = Vec::new();
        for y in 0..self.size {
            for x in 0..self.size {
                let coords = Coords::new(x, y);
                if self.cells.get_cell(&coords)? == Cell::Empty {
                    empty_cells.push(coords);
                }
            }
        }

        Ok(empty_cells)
    }

    pub fn check_legal_cell(&self, coords: &Coords) -> Result<bool, &'static str> {
        Ok(self.check_legal_horizontal(coords)? && self.check_legal_vertical(coords)?)
    }

    fn check_legal_horizontal(&self, coords: &Coords) -> Result<bool, &'static str> {
        // Check if wall conditions are met
        if coords.x > 0 {
            let left_wall = self.walls.get_wall(&FromCellCoords::Left(coords.clone()))?;
            let left_cell = self.cells.get_cell(&Coords::new(coords.x - 1, coords.y))?;
            let cell = self.cells.get_cell(coords)?;

            if !left_wall.check_cells(&cell, &left_cell) {
                return Ok(false);
            }
        }
        if coords.x < self.size - 1 {
            let right_cell = self.cells.get_cell(&Coords::new(coords.x + 1, coords.y))?;
            if !right_cell.is_empty() {
                let right_wall = self.walls.get_wall(&FromCellCoords::Left(Coords::new(coords.x + 1, coords.y)))?;
                let cell = self.cells.get_cell(coords)?;

                if !right_wall.check_cells(&cell, &right_cell) {
                    return Ok(false);
                }
            }
        }

        // Check if we have more than 2 consecutive cells of the same type horizontally
        let mut prev_cell;
        let mut cont = 0;

        prev_cell = Cell::Empty;
        for x in coords.x.saturating_sub(2)..=u32::min(coords.x + 2, self.size - 1) {
            let current_coords = Coords::new(x, coords.y);
            let current_cell = self.cells.get_cell(&current_coords)?;

            if current_cell == prev_cell {
                cont += 1;
                if cont > 2 && !current_cell.is_empty() {
                    return Ok(false);
                }
            }
            else {
                cont = 1;
                prev_cell = current_cell;
            }
        }

        // Check if the amount of black and white cells is the same, only if the row is full
        let mut white_count = 0;
        let mut black_count = 0;
        let mut is_full = true;
        for x in 0..self.size {
            let current_coords = Coords::new(x, coords.y);
            let current_cell = self.cells.get_cell(&current_coords)?;
            match current_cell {
                Cell::Empty => { 
                    is_full = false;
                    break;
                }
                Cell::Black => {
                    black_count += 1;
                    if black_count > self.size / 2 {
                        return Ok(false);
                    }
                },
                Cell::White => {
                    white_count += 1;
                    if white_count > self.size / 2 {
                        return Ok(false);
                    }
                },
            };
        }

        if is_full && white_count != black_count {
            return Ok(false);
        }


        Ok(true)
    }

    fn check_legal_vertical(&self, coords: &Coords) -> Result<bool, &'static str> {
        // Check if wall conditions are met
        if coords.y > 0 {
            let above_wall = self.walls.get_wall(&FromCellCoords::Up(coords.clone()))?;
            let above_cell = self.cells.get_cell(&Coords::new(coords.x, coords.y - 1))?;
            let cell = self.cells.get_cell(coords)?;

            if !above_wall.check_cells(&cell, &above_cell) {
                return Ok(false);
            }
        }
        if coords.y < self.size - 1 {
            let below_cell = self.cells.get_cell(&Coords::new(coords.x, coords.y + 1))?;
            if !below_cell.is_empty() {
                let below_wall = self.walls.get_wall(&FromCellCoords::Up(Coords::new(coords.x, coords.y + 1)))?;
                let cell = self.cells.get_cell(coords)?;

                if !below_wall.check_cells(&cell, &below_cell) {
                    return Ok(false);
                }
            }
        }

        // Check if we have more than 2 consecutive cells of the same type vertically
        let mut prev_cell;
        let mut cont = 0;

        prev_cell = Cell::Empty;
        for y in coords.y.saturating_sub(2)..=u32::min(coords.y + 2, self.size - 1) {
            let current_coords = Coords::new(coords.x, y);
            let current_cell = self.cells.get_cell(&current_coords)?;

            if current_cell == prev_cell {
                cont += 1;
                if cont > 2 && !current_cell.is_empty() {
                    return Ok(false);
                }
            }
            else {
                cont = 1;
                prev_cell = current_cell;
            }
        }

        // Check if the amount of black and white cells is the same, only if the column is full
        let mut white_count = 0;
        let mut black_count = 0;
        let mut is_full = true;
        for y in 0..self.size {
            let current_coords = Coords::new(coords.x, y);
            let current_cell = self.cells.get_cell(&current_coords)?;
            match current_cell {
                Cell::Empty => {
                    is_full = false;
                    break;
                }
                Cell::Black => {
                    black_count += 1;
                    if black_count > self.size / 2 {
                        return Ok(false);
                    }
                },
                Cell::White => {
                    white_count += 1;
                    if white_count > self.size / 2 {
                        return Ok(false);
                    }
                },
            };
        }

        if is_full && white_count != black_count {
            return Ok(false);
        }

        Ok(true)
    }

    pub fn print_board(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                let coords = Coords::new(x, y);
                let cell = self.cells.get_cell(&coords).unwrap();
                match cell {
                    Cell::Empty => print!(". "),
                    Cell::Black => print!("B "),
                    Cell::White => print!("W "),
                };
            }
            println!();
        }
    }

    pub fn cells(&self) -> &Vec<Cell> {
        self.cells.cells()
    }
}
