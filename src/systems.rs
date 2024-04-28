use crate::prelude::*;
pub fn run_systems(state: &mut State) {
    //then look to see what numbers are in the direct big square
    //find that out via modulus or something maybe idk or just hardcode it. whatever.
    //remove any numbers in those regions from the potential numbers
    //then print out the sudoku board
    //if all cells are solved flag completed (maybe?)
    //then prompt user for input before closing

    //immutable copy of the board before any actions are taken on it to reference it for checking cell possibilities
    let board_ref = state.soduku_board.cells.clone();
    let mut idx = 0;
    for cell in state.soduku_board.cells.iter_mut() {
        match cell {
            //if
            CellOption::Unsolved(options) => {
                //first check to see if there's only one possible number left and if so set it to solved
                //using the only remaining celltype.
                if options.len() == 1 {
                    let celltype_buffer = options[0];
                    *cell = CellOption::Solved(celltype_buffer);
                }
                //create a default collection of all possibilities that will then be pared down
                let mut options_buffer = vec![
                    CellType::One,
                    CellType::Two,
                    CellType::Three,
                    CellType::Four,
                    CellType::Five,
                    CellType::Six,
                    CellType::Seven,
                    CellType::Eight,
                    CellType::Nine,
                ];
                //get the cell's position as a point to make checking geometric relationships to other cells easier
                let cell_pos = idx_to_pos(idx);
                //first check if there's any solved cells on the current cells X axis
                for x in 0..9 {
                    let ref_idx = pos_to_idx(Point::new(x, cell_pos.y));
                    match board_ref[ref_idx] {
                        CellOption::Unsolved(_) => {
                            //if the cell is also unsolved you don't need to do anything
                        }
                        //if the cell is solved then remove that cell number from the list of possibilities
                        CellOption::Solved(ref_celltype) => {
                            //index tracking variable
                            let mut option_idx = 0;
                            //stores the index of the number that needs to be removed from the list of options
                            let mut option_to_remove: Option<usize> = None;
                            //go through the buffer of all possible numbers
                            for option in options_buffer.iter() {
                                //if the number in the solved cell is the same as one still available in the list of options
                                //break out of the loop and store the index
                                if option == &ref_celltype {
                                    option_to_remove = Some(option_idx);
                                    break;
                                }
                                option_idx += 1;
                            }
                            //remove the existing number from the list of options using the index we've generated.
                            if option_to_remove.is_some() {
                                options_buffer.remove(option_to_remove.unwrap());
                            }
                        }
                    }
                }
                //next check if there's any solved cells on the current cells Y axis
                for y in 0..9 {
                    let ref_idx = pos_to_idx(Point::new(cell_pos.x, y));
                    match board_ref[ref_idx] {
                        CellOption::Unsolved(_) => {
                            //if the cell is also unsolved you don't need to do anything
                        }
                        //if the cell is solved then remove that cell number from the list of possibilities
                        CellOption::Solved(ref_celltype) => {
                            //index tracking variable
                            let mut option_idx = 0;
                            //stores the index of the number that needs to be removed from the list of options
                            let mut option_to_remove: Option<usize> = None;
                            //go through the buffer of all possible numbers
                            for option in options_buffer.iter() {
                                //if the number in the solved cell is the same as one still available in the list of options
                                //break out of the loop and store the index
                                if option == &ref_celltype {
                                    option_to_remove = Some(option_idx);
                                    break;
                                }
                                option_idx += 1;
                            }
                            //remove the existing number from the list of options using the index we've generated.
                            if option_to_remove.is_some() {
                                options_buffer.remove(option_to_remove.unwrap());
                            }
                        }
                    }
                }
                //next check the 3x3 squares within the soduku board to see if there are any solved squares in there
                //what are the bounds of those 9 squares expressed as point ranges

                /*
                Big Square 0,0
                0,0 -> 2,2
                OR
                1,1 -> 3,3
                Big Square 1,0
                3,0 -> 5,2
                OR
                4,1 -> 6,3
                Big Square 2,0
                6,0 -> 8,2
                OR
                7,1 -> 9,3
                Big Square 0,1
                0,3 -> 2,5
                OR
                1,4 -> 3,6
                Big Square 1,1
                3,3 -> 5,5
                OR
                4,4 -> 6,6
                Big Square 2,1

                Big Square 0,2

                Big Square 1,2

                Big Square 2,2
                */

                //do all the steps necessary for a solving one
            }
            CellOption::Solved(_) => {
                //don't need to do anything
            }
        }
        idx += 1;
    }
}

///Converts a 2D Point into a usize index for accessing an 81 cell sized sudoku board
fn pos_to_idx(pos: Point) -> usize {
    ((pos.y * 9) + pos.x) as usize
}
///Converts a usize index used for accessing an 81 cell sized sudoku board into a 2D Point
fn idx_to_pos(idx: usize) -> Point {
    let mut pos = Point::new(0, 0);
    while idx > 9 {
        pos.y += 1;
        idx - 9;
    }
    pos.x = idx as i32;
    pos
}
/*
╔═════════════════╗
║0x8x7║0 0 0X6x9x0║
║─────║───────────║
║0x0x9║0 6 3X7x0x0║
║─────║───────────║
║0x0x0║7 0 5X0x0x0║
║─────────────────║
║0x7x6X0 0 9X1x0x0║
───────────────────
║0x1x0X0 0 0X0x4x0║
───────────────────
║0x0x8X1 0 0X3x7x0║
───────────────────
║0x0x0X4 0 7X0x0x0║
───────────────────
║0x0x4X3 5 0X2x0x0║
───────────────────
║0x2x5X0 0 0X4x1x0║
╚═════════════════╝
*/
