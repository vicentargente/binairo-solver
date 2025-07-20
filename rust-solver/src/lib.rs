use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::board::{enums::{cell::CellCoords, wall::WallCoords}, Board};

mod board;
mod solve; 

#[wasm_bindgen]
pub fn solve_from_js(size: u32, initial_cells: JsValue, initial_walls: JsValue) -> Result<JsValue, JsValue> {
    let cells: Vec<CellCoords> = serde_wasm_bindgen::from_value(initial_cells)
        .map_err(|e| e.to_string())?;

    let walls: Vec<WallCoords> = serde_wasm_bindgen::from_value(initial_walls)
        .map_err(|e| e.to_string())?;

    let mut board = Board::from_cells_and_walls(size, cells, walls)
        .map_err(|e| JsValue::from_str(e))?;

    solve::solve(&mut board).map_err(|e| JsValue::from_str(e))?;

    let solved_cells = board.cells();
    serde_wasm_bindgen::to_value(solved_cells).map_err(|e| e.to_string().into())
}
