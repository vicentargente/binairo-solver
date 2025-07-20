use crate::board::{coords::Coords, enums::{cell::FromCellCoords, wall::{Wall, WallCoords}}};

pub struct Walls {
    size: u32,
    vertical: Vec<Wall>, // Left || Right
    horizontal: Vec<Wall> // Up || Down
}

impl Walls {
    pub fn new(size: u32) -> Self {
        let size_usize = size as usize;
        let walls_per_direction = size_usize * (size_usize - 1);

        Walls {
            size,
            vertical: vec![Wall::Normal; walls_per_direction],
            horizontal: vec![Wall::Normal; walls_per_direction]
        }
    }

    pub fn from_walls(size: u32, walls: Vec<WallCoords>) -> Result<Self, &'static str> {
        let mut walls_grid = Walls::new(size);
        for WallCoords { coords, wall } in walls {
            walls_grid.set_wall(&coords, wall)?;
        }

        Ok(walls_grid)
    }

    pub fn get_wall(&self, coords: &FromCellCoords) -> Result<Wall, &'static str> {
        match coords {
            FromCellCoords::Up(coords) => {
                let index = self.get_above_wall_index(&coords)?;
                Ok(self.horizontal[index])
            },
            FromCellCoords::Left(coords) => {
                let index = self.get_left_wall_index(&coords)?;
                Ok(self.vertical[index])
            }
        }
    }

    pub fn set_wall(&mut self, coords: &FromCellCoords, wall: Wall) -> Result<(), &'static str> {
        match coords {
            FromCellCoords::Up(coords) => {
                let index = self.get_above_wall_index(&coords)?;
                self.horizontal[index] = wall;
            },
            FromCellCoords::Left(coords) => {
                let index = self.get_left_wall_index(&coords)?;
                self.vertical[index] = wall;
            }
        };

        Ok(())
    }

    fn get_above_wall_index(&self, coords: &Coords) -> Result<usize, &'static str> {
        if coords.y == 0 {
            return Err("Cannot check wall above the top row");
        }

        let coords = Coords::new(coords.x, coords.y - 1);
        let index = coords.to_index(self.size);
        if index >= self.horizontal.len() {
            return Err("Above index out of bounds");
        }

        Ok(index)
    }

    fn get_left_wall_index(&self, coords: &Coords) -> Result<usize, &'static str> {
        if coords.x == 0 {
            return Err("Cannot check wall to the left of the first column");
        }

        let coords = Coords::new(coords.x - 1, coords.y);
        let index = coords.to_index(self.size - 1);
        if index >= self.vertical.len() {
            return Err("Left index out of bounds");
        }

        Ok(index)
    }
}
