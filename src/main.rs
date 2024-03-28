use std::collections::HashMap;
use rand::seq::SliceRandom;

#[derive(Debug)]
struct Symbol {
    color: (u8,u8,u8),
    size: &'static str,
    stype: &'static str,
}

#[derive(Debug)]
struct Stone {
    radius: f32,
    symbols: [Symbol; 4]
}

struct Board {
    width: usize,
    height: usize,
    samount: usize,
    stones: HashMap<(usize,usize),Stone> // Tuple with Coords
}

impl Symbol {
    fn new() -> Symbol {
        Symbol {
            color: Symbol::random_item(&mut [(0, 0, 0), (255, 0, 0), (0, 255, 0), (0, 0, 255)]),
            size: Symbol::random_item(&mut ["large", "medium", "small"]),
            stype: Symbol::random_item(&mut ["puzzle", "mushroom", "heart", "leaf"]),
        }
    }

    fn random_item<T>(items: &mut [T]) -> T {
        let mut rng = rand::thread_rng();
        items.shuffle(&mut rng);
        *items.choose(&mut rng).unwrap()
    }
}

impl Stone {
    fn new() -> Stone { // no input required, random placement
        Stone {
            radius: 25.0, // 50px initially TODO: make this a function of the amount of stones (maybe dependent on players and the size of the board)
            symbols: Stone::add_symbols() // TODO: make sure the sizes, colors, and types are evenly distributed on each stone
        }
    }

    fn add_symbols() -> [Symbol;4] {
        let mut symbols = [Symbol::new(), Symbol::new(), Symbol::new(), Symbol::new()];
        symbols
    }
}

impl Board {
    fn new(width: usize, height: usize, samount: usize) -> Board {
        Board {
            width,
            height,
            samount,
            stones: HashMap::new(),
        }
    }

    fn place_stones(&mut self) {
        // Randomly place stones without overlapping
        let mut rng = rand::thread_rng();

        for _ in 0..self.samount {
            let x = rng.gen_range(0, self.width);
            let y = rng.gen_range(0, self.height);

            // Ensure the position is not occupied
            while self.stones.contains_key(&(x, y)) {
                let x = rng.gen_range(0, self.width);
                let y = rng.gen_range(0, self.height);
            }

            self.stones.insert((x, y), Stone::new());
        }
    }
}

fn main() {
    let mut board = Board::new(800, 500, 24);
    board.place_stones();
    for (key, value) in &board.stones {
        println!("{:?} - {:?}", key, value);
    }
}

