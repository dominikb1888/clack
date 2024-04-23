use std::collections::HashMap;

use web_time::{Instant, SystemTime};
use rand::prelude::SliceRandom;
use rand::Rng;

const COLORS: [(u8,u8,u8); 5]= [(0, 255, 255), (255, 0, 0), (0, 255, 0), (0, 0, 255), (255, 255, 0)];
const SHAPES: [&str; 5] = ["puzzle", "mushroom", "heart", "leaf", "arrow"];

#[derive(Debug)]
pub struct Symbol {
    color: (u8,u8,u8),
    size: &'static str,
    stype: &'static str,
}

#[derive(Debug)]
pub struct Stone {
    pub color: (u8,u8,u8),
    pub radius: usize,
    symbols: [Symbol; 3]
}

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub samount: usize,
    pub stones: HashMap<(usize,usize),Stone>
}

impl Symbol {
    fn new() -> Symbol {
        Symbol {
            color: Symbol::random_item(&mut COLORS),
            size: Symbol::random_item(&mut ["large", "medium", "small"]),
            stype: Symbol::random_item(&mut SHAPES),
        }
    }

    fn random_item<T: Clone>(items: &[T]) -> T {
        let mut rng = rand::thread_rng();
        items.choose(&mut rng).unwrap().clone()
    }
}

impl Stone {
    fn new() -> Stone { // no input required, random placement
        Stone {
            radius: 25, // 50px initially TODO: make this a function of the amount of stones (maybe dependent on players and the size of the board)
            symbols: Stone::add_symbols(), // TODO: make sure the sizes, colors, and types are evenly distributed on each stone
            color: Symbol::random_item(&mut COLORS),
        }
    }

    fn add_symbols() -> [Symbol;3] {
        let symbols = [Symbol::new(), Symbol::new(), Symbol::new()];
        symbols
    }
}

impl Board {
    pub fn new(width: usize, height: usize, samount: usize) -> Board {
        Board {
            width,
            height,
            samount,
            stones: HashMap::new(),
        }
    }

    pub fn place_stones(&mut self) {
    // Randomly place stones without overlapping
        let mut rng = rand::thread_rng();

        for _ in 0..self.samount {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);

            // Ensure the position is not occupied
            while self.stones.contains_key(&(x, y)) {
                let x = rng.gen_range(0..self.width * 2);
                let y = rng.gen_range(0..self.height * 2);
            }

            self.stones.insert((x, y), Stone::new());
        }
    }
}

//
// impl Scoreboard {
//     pub fn from_board(board: &Board) -> Self {
//         let mut scoreboard_data = ScoreboardData::default();
//         for (coordinates, _) in &board.stones {
//             scoreboard_data.stone_clicks.insert(*coordinates);
//         }
//         scoreboard_data
//     }
//
//      fn update_score(&mut self, player: &str) {
//         *self.scores.entry(player.to_string()).or_insert(0) += 1;
//     }
//
//     pub fn put_on_stack(&mut self, coords: (usize,usize), time: SystemTime, hostname: String) -> () {
//     // Decide whether a click was -> How can we be aware of the current dice?
//     // a. on the right stone, given the current dice
//     // b. before any other players click, if two players clicked on the same stone at about the
//     // same time
//     // TODO: Would be nice to have a count of all Symbols on all stones as reference. For this
//     // using the actual stones is necessary
//     //
//         ()
//
//     }
//
//     pub fn throw_dice() -> (String, String) {
//         let color_dice = &mut COLORS.to_vec();
//         let shape_dice = &mut SHAPES.to_vec();
//         color_dice.push("Empty");
//         shape_dice.push("Empty");
//
//         (Symbol::random_item(color_dice), Symbol::random_item(shape_dice))
//     }
//
//     // Duplicate of Symbol... Trait? Random?
//     fn random_item<T: Clone>(items:&Vec<String>) -> String {
//         let mut rng = rand::thread_rng();
//         items.choose(&mut rng).unwrap().clone()
//     }

// }
