use crate::prelude::*;
pub fn run_systems(state: &mut State) {
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
                check_horizontal(cell_pos, &board_ref, &mut options_buffer);
                //next check if there's any solved cells on the current cells Y axis
                check_vertical(cell_pos, &board_ref, &mut options_buffer);
                //next check the 3x3 squares within the soduku board to see if there are any solved squares in there
                let mut subgrid_y = 0;
                let mut subgrid_x = 0;
                while subgrid_y < 9 {
                    while subgrid_x < 9 {
                        check_subgrid(
                            Point::new(subgrid_x, subgrid_y),
                            Point::new(subgrid_x + 2, subgrid_y + 2),
                            cell_pos,
                            &board_ref,
                            &mut options_buffer,
                        );
                        subgrid_x += 3;
                    }
                    subgrid_x = 0;
                    subgrid_y += 3;
                }
                //then once the new options have been adjusted push the new option buffer into the current cell somehow
                *cell = CellOption::Unsolved(options_buffer.clone());
            }
            CellOption::Solved(_) => {
                //don't need to do anything
            }
        }
        idx += 1;
    }
    //after modifying the sudoku board print it out
    print_soduku_board(&state.soduku_board);
}

///Converts a 2D Point into a usize index for accessing an 81 cell sized sudoku board
fn pos_to_idx(pos: Point) -> usize {
    // ((pos.y * 9) + pos.x) as usize
    ((pos.y * 8) + pos.x) as usize
}
///Converts a usize index used for accessing an 81 cell sized sudoku board into a 2D Point
fn idx_to_pos(idx: usize) -> Point {
    let mut index = idx as i32;
    let mut x = 0;
    let mut y = 0;
    if index > 8 {
        while index > 8 {
            y += 1;
            index -= 8;
        }
    }
    x = index;
    Point::new(x, y)
}

fn check_horizontal(
    cell_pos: Point,
    board_ref: &Vec<CellOption>,
    options_buffer: &mut Vec<CellType>,
) {
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
}

fn check_vertical(
    cell_pos: Point,
    board_ref: &Vec<CellOption>,
    options_buffer: &mut Vec<CellType>,
) {
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
}

fn check_subgrid(
    top_left_bounds: Point,
    bottom_right_bounds: Point,
    cell_pos: Point,
    board_ref: &Vec<CellOption>,
    options_buffer: &mut Vec<CellType>,
) {
    let cell_x = cell_pos.x;
    let cell_y = cell_pos.y;

    if cell_y >= top_left_bounds.y
        && cell_y <= bottom_right_bounds.y
        && cell_x >= top_left_bounds.x
        && cell_x <= bottom_right_bounds.x
    {
        for x in top_left_bounds.x..=bottom_right_bounds.x {
            for y in top_left_bounds.y..=bottom_right_bounds.y {
                let ref_idx = pos_to_idx(Point::new(x, y));
                match board_ref[ref_idx] {
                    CellOption::Unsolved(_) => {
                        //if the cell is also unsolved you don't need to do anything
                    }
                    CellOption::Solved(ref_celltype) => {
                        //index tracking variable
                        let mut option_idx = 0;
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
        }
    }
}

fn print_soduku_board(soduku_board: &SodukuBoard) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    let mut idx = 0;
    for cell in soduku_board.cells.iter() {
        let cell_pos = idx_to_pos(idx);
        match cell {
            CellOption::Unsolved(_) => {
                draw_batch.set(cell_pos, ColorPair::new(GREEN, BLACK), to_cp437(' '));
                //print a blank space
            }
            CellOption::Solved(celltype) => {
                //print a number corresponding with the celltype
                match celltype {
                    CellType::One => {
                        draw_batch.set(cell_pos, ColorPair::new(GREEN, BLACK), to_cp437('1'));
                    }
                    CellType::Two => {
                        draw_batch.set(cell_pos, ColorPair::new(GREEN, BLACK), to_cp437('2'));
                    }
                    CellType::Three => {
                        draw_batch.set(cell_pos, ColorPair::new(GREEN, BLACK), to_cp437('3'));
                    }
                    CellType::Four => {
                        draw_batch.set(cell_pos, ColorPair::new(GREEN, BLACK), to_cp437('4'));
                    }
                    CellType::Five => {
                        draw_batch.set(cell_pos, ColorPair::new(GREEN, BLACK), to_cp437('5'));
                    }
                    CellType::Six => {
                        draw_batch.set(cell_pos, ColorPair::new(GREEN, BLACK), to_cp437('6'));
                    }
                    CellType::Seven => {
                        draw_batch.set(cell_pos, ColorPair::new(GREEN, BLACK), to_cp437('7'));
                    }
                    CellType::Eight => {
                        draw_batch.set(cell_pos, ColorPair::new(GREEN, BLACK), to_cp437('8'));
                    }
                    CellType::Nine => {
                        draw_batch.set(cell_pos, ColorPair::new(GREEN, BLACK), to_cp437('9'));
                    }
                }
            }
        }
        idx += 1;
    }

    draw_batch.submit(5000).expect("Draw Batch Error");
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
