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
            symbols: Stone::addSymbols() // TODO: make sure the sizes, colors, and types are evenly distributed on each stone
        }
    }

    fn addSymbols() -> [Symbol;4] {
        let mut symbols = [Symbol::new(), Symbol::new(), Symbol::new(), Symbol::new()];
        symbols
    }
}

impl Board {
    fn new(width: usize, height: usize, samount: usize) -> Board {
        Board {
            width: width,
            height: height,
            samount: samount, // TODO: make this a function of the amount of players
            stones: HashMap::new(),
        } // create a set of random stones across the board
    }

    fn place_stones(&mut self) {
        // TODO: create a list of coordinates based on the size of each Stone which is randomly
        // distributed, but not overlapping... let's start with a matrix first.

        // split the size of the board into equal regions based on smaount and get the coordinate
        // in its middle to place the center of the stone there
        // TODO: evaluate, if this needs to be in the backend or frontend, iteration 1: Backend
        // Calculate the number of rows and columns

        let nrows = (self.samount as f64 / (self.width as f64 / self.height as f64)).sqrt().ceil() as usize;
        let ncols = (self.samount as f64 / (self.height as f64 / self.width as f64)).sqrt().ceil() as usize;

        for i in 1..=ncols {
            for j in 1..=nrows {
                let x = (i * (self.samount/ncols) - (self.samount/ncols)/2) as usize;
                let y = (j * (self.samount/nrows) - (self.samount/nrows)/2) as usize;
                self.stones.insert((x,y), Stone::new());
            }
        }
    }
}

fn main() {
    let mut board = Board::new(800, 500, 24);
    board.place_stones();
    for (key, value) in board.stones.into_iter() {
        println!("{:?} - {:?}", key, value);
    }
}
