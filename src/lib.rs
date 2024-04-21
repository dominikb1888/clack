use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Debug)]
pub struct Symbol {
    color: (u8, u8, u8),
    size: &'static str,
    stype: &'static str,
}

#[derive(Debug)]
pub struct Stone {
    pub color: (u8, u8, u8),
    pub radius: usize,
    symbols: [Symbol; 3],
}

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub samount: usize,
    pub stones: HashMap<(usize, usize), Stone>,
}

impl Symbol {
    fn new() -> Symbol {
        Symbol {
            color: Symbol::random_item(&mut [
                (0, 255, 255),
                (255, 0, 0),
                (0, 255, 0),
                (0, 0, 255),
                (255, 255, 0),
            ]),
            size: Symbol::random_item(&mut ["large", "medium", "small"]),
            stype: Symbol::random_item(&mut ["puzzle", "mushroom", "heart", "leaf", "arrow"]),
        }
    }

    fn random_item<T: Clone>(items: &[T]) -> T {
        let mut rng = rand::thread_rng();
        items.choose(&mut rng).unwrap().clone()
    }
}

impl Stone {
    fn new() -> Stone {
        // no input required, random placement
        Stone {
            radius: 25, // 50px initially TODO: make this a function of the amount of stones (maybe dependent on players and the size of the board)
            symbols: Stone::add_symbols(), // TODO: make sure the sizes, colors, and types are evenly distributed on each stone
            color: Symbol::random_item(&mut [(255, 0, 0), (0, 255, 0), (0, 0, 255)]),
        }
    }

    fn add_symbols() -> [Symbol; 3] {
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
                let x = rng.gen_range(0..self.width);
                let y = rng.gen_range(0..self.height);
            }

            self.stones.insert((x, y), Stone::new());
        }
    }
}
