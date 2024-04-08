use std::collections::HashMap;
use rand::seq::SliceRandom;

#[derive(Debug)]
struct Symbol {
    color: (u8,u8,u8),
    size: &'static str,
    shape: &'static str,
}

#[derive(Debug)]
struct Piece {
    radius: f32,
    symbols: [Symbol; 3], // Symbol Position will be handled on Frontend
    color: (u8,u8,u8),
}

struct Board {
    width: usize,
    height: usize,
    samount: usize,
    color: (u8,u8,u8),
    stones: HashMap<(usize,usize),Piece> // Tuple with Coords - just for now. Also on Frontend
    // later
}

impl Symbol {
    fn new() -> Symbol { // no input required, random symbols in range of colors
        Symbol {
            color: Symbol::randomColor(),
            size: Symbol::randomSize(),
            shape: Symbol::randomShape(),
        }
    }

    //TODO: Let user set colors via config, loading and validation needed
    fn randomColor() -> (u8,u8,u8) {
        let mut rng = rand::thread_rng();
        let colors: [(u8,u8,u8);3] = [(255,0,0),(0,255,0),(0,0,255)]; // Color theming? What about pastel colors?
        *colors.choose(&mut rng).unwrap()
    }

    fn randomShape() -> &'static str {
        let mut rng = rand::thread_rng();
        let stypes = ["puzzle","mushroom","heart","leaf"];
        *stypes.choose(&mut rng).unwrap()
    }

    fn randomSize() -> &'static str {
        let mut rng = rand::thread_rng();
        let sizes = ["large","medium","small"]; // TODO: What happens if colors or sizes are taken?
        *sizes.choose(&mut rng).unwrap()
    }
}

impl Piece {
    fn new() -> Piece { // no input required, random placement
        Piece {
            color: (255,255,255), // TODO: Dark mode? What up?!
            radius: 25.0, // 50px initially TODO: make this a function of the amount of stones (maybe dependent on players and the size of the board)
            symbols: Piece::addSymbols() // TODO: make sure the sizes, colors, and types are evenly distributed on each stone
        }
    }

    fn addSymbols() -> [Symbol;3] {
        let mut symbols = [Symbol::new(), Symbol::new(), Symbol::new()];
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
            color: (0,0,0),
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
                self.stones.insert((x,y), Piece::new());
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
