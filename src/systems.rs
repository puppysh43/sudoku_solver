use crate::{prelude::*, CellOption, CellType};
pub fn run_systems(state: &mut State) {
    //iterate through all cells in the sudoku board
    //if a cell in the board is unsolved then run the checks on it
    //there will be generated a list of potential numbers that includes all 9
    //first look to see what numbers are in the y axis of the cell
    //then look to see what numbers are in the x axis of the cell
    //then look to see what numbers are in the direct big square
    //find that out via modulus or something maybe idk or just hardcode it. whatever.
    //remove any numbers in those regions from the potential numbers
    //once all the potential ones have been updated check to see which ones of them have a vec length of 1
    //and switch it to a solved cell
    //then print out the sudoku board
    //if all cells are solved flag completed (maybe?)
    //then prompt user for input before closing
    for cell in state.soduku_board.cells.iter_mut() {
        //nothing
        match cell {
            CellOption::Unsolved(possibilities) => {
                if possibilities.len() == 1 {
                    let celltype_buffer = possibilities[0];
                    cell = CellOption::Solved(celltype_buffer);
                }
                //do all the steps necessary for a solving one
            }
            CellOption::Solved(number) => {
                //don't need to do anything
            }
        }
    }
}

/*
╔═════════════════╗
║0x8x7X0 0 0X6x9x0║
║─────────────────
║0x0x9X0 6 3X7x0x0║
───────────────────
║0x0x0X7 0 5X0x0x0║
───────────────────
║0x7x6X0 0 9X1x0x0║
║0x1x0X0 0 0X0x4x0║
║0x0x8X1 0 0X3x7x0║
║0x0x0X4 0 7X0x0x0║
║0x0x4X3 5 0X2x0x0║
║0x2x5X0 0 0X4x1x0║
╚═════════════════╝
*/
