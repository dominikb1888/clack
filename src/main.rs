use clack::Board;

fn main() {
    let mut board = Board::new(800, 500, 24);
    board.place_stones();
    for (key, value) in &board.stones {
        println!("{:?} - {:?}", key, value);
    }
}

