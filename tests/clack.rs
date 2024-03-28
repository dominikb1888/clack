
#[cfg(test)]
mod tests {
    use clack::Board;

    #[test]
    fn test_new_board() {
        let board = Board::new(800, 500, 24);
        assert_eq!(board.width, 800);
        assert_eq!(board.height, 500);
        assert_eq!(board.samount, 24);
        assert!(board.stones.is_empty());
    }

    #[test]
    fn test_place_stones() {
        let mut board = Board::new(800, 500, 24);
        board.place_stones();

        assert_eq!(board.stones.len(), 24); // Ensure correct number of stones

        // Ensure stones are placed within the board boundaries
        for (&(x, y), _) in &board.stones {
            assert!(x < board.width);
            assert!(y < board.height);
        }

        // Ensure stones are not overlapping
        for (coord1, _) in &board.stones {
            for (coord2, _) in &board.stones {
                if coord1 != coord2 {
                    assert_ne!(coord1, coord2);
                }
            }
        }
    }
}

