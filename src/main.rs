/*
Use this for generating random sudoku puzzles
https://www.menneske.no/sudoku/eng/random.html
*/
mod prelude {
    pub use crate::State;
}

mod systems;

use bracket_terminal::prelude::*;
use std::fs;

//soduku board is composed of 9 big squares (3x3) with each big square composed of
//3x3 little squares
//81 squares

#[derive(PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    VeryHard,
    Impossible,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CellOption {
    Unsolved(Vec<CellType>),
    Solved(CellType),
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CellType {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
pub struct SodukuBoard {
    pub cells: Vec<CellOption>,
}

impl SodukuBoard {
    pub fn new(difficulty: Difficulty) -> Self {
        let filepath = String::new();
        let filepath = match difficulty {
            Difficulty::Easy => "raws/easy.txt".to_string(),
            Difficulty::Medium => "raws/medium.txt".to_string(),
            Difficulty::Hard => "raws/hard.txt".to_string(),
            Difficulty::VeryHard => "raws/very_hard.txt".to_string(),
            Difficulty::Impossible => "impossible.txt".to_string(),
        };
        let mut raw_soduku =
            fs::read_to_string(filepath).expect("failed to read specified soduku raw");
        raw_soduku.retain(|c| !c.is_whitespace());
        raw_soduku.retain(|c| c.is_numeric());
        let soduku_data: Vec<char> = raw_soduku.chars().collect();
        let mut cells: Vec<CellOption> = Vec::new();
        if soduku_data.len() == 81 {
            for c in soduku_data.iter() {
                match c {
                    '0' => cells.push(CellOption::Unsolved(Vec::new())),
                    '1' => cells.push(CellOption::Solved(CellType::One)),
                    '2' => cells.push(CellOption::Solved(CellType::Two)),
                    '3' => cells.push(CellOption::Solved(CellType::Three)),
                    '4' => cells.push(CellOption::Solved(CellType::Four)),
                    '5' => cells.push(CellOption::Solved(CellType::Five)),
                    '6' => cells.push(CellOption::Solved(CellType::Six)),
                    '7' => cells.push(CellOption::Solved(CellType::Seven)),
                    '8' => cells.push(CellOption::Solved(CellType::Eight)),
                    '9' => cells.push(CellOption::Solved(CellType::Nine)),
                    _ => println!("Can't read character in sudoku raw."), //nothing,
                }
            }
        }
        Self { cells }
    }
}

#[derive(Clone, Debug)]
pub struct State {
    //soduku puzzle state as its being solved
    soduku_board: SodukuBoard,
    key: Option<VirtualKeyCode>,
    completed: bool, //tracks if the soduku puzzle has been solved yet
}
impl State {
    pub fn new(difficulty: Difficulty) -> Self {
        //take raw path and use it to grab a specific file in the raw directory
        //pass that to the new function for the soduku board
        //initialize the key and completed state
        Self {
            soduku_board: SodukuBoard::new(difficulty),
            key: None,
            completed: false,
        }
    }
}
//array of sudoku cells that have functions to get point from usize and etc
//enum for solved and unsolved in cell
//maybe have soduku number become an enum for absolute type safety?
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        self.key = ctx.key;
        systems::run_systems(self);
        render_draw_buffer(ctx).expect("Render Error")
        //iterate through the soduku puzzle and for unsolved squares check all neighbors
        //use this to select options, when there's only one option set it to solved
    }
}
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Automatic Soduku Solver")
        .build()?;
    main_loop(context, State::new(Difficulty::Impossible))
}
