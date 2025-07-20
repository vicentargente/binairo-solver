use crate::board::{enums::cell::Cell, Board};


pub fn solve(board: &mut Board) -> Result<(), &'static str> {
    let empty_cells = board.get_empty_cells()?;
    if empty_cells.is_empty() {
        return Ok(());
    }

    let mut empty_cell_index = 0;
    while empty_cell_index < empty_cells.len() {
        let coords = empty_cells.get(empty_cell_index).unwrap();
        let mut cell = board.get_cell(coords)?;

        let mut found_legal_state = false;
        while !cell.get_next_state().is_empty() {
            let next_cell_state = cell.get_next_state();

            board.set_cell(coords, next_cell_state)?;
            if board.check_legal_cell(coords)? {
                empty_cell_index += 1;
                found_legal_state = true;
                break;
            }

            cell = next_cell_state;
        }

        if !found_legal_state {
            if empty_cell_index == 0 {
                return Err("No solution found");
            }

            board.set_cell(coords, Cell::Empty)?;
            empty_cell_index -= 1;
            continue;
        }
    }

    Ok(())
}
