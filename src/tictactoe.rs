use rand::prelude::*;

/// Two players, X and O. X always goes first.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Player {
    X,
    O,
}

impl From<Square> for Player {
    fn from(s: Square) -> Self {
        match s {
            Square::Occupied(Player::X) => Player::X,
            Square::Occupied(Player::O) => Player::O,
            Square::Empty => panic!(),
        }
    }
}

/// A square on a board, either occupied by a player or empty
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Square {
    Occupied(Player),
    Empty,
}

/// A 3x3 tic-tac-toe board
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Board([[Square; 3]; 3]);
impl Board {
    /// Return an empty board
    pub fn empty() -> Board {
        Board([[Square::Empty; 3]; 3])
    }

    /// Return a random board where x is about to move,
    /// and nobody has won yet.
    pub fn random_nonterminal_x_board() -> Board {
        let mut rng = rand::thread_rng();

        // Function that just generates some random board
        fn random_board(rng: &mut ThreadRng) -> Board {
            let num_pieces = rng.gen_range(0..=4) * 2;
            let mut board = Board::empty();
            let mut pieces = [
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 1),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2),
            ];
            pieces.shuffle(rng);
            for (i, p) in pieces[0..num_pieces].iter().enumerate() {
                board.0[p.0][p.1] = if i % 2 == 0 {
                    Square::Occupied(Player::X)
                } else {
                    Square::Occupied(Player::O)
                };
            }
            board
        }

        let mut board = random_board(&mut rng);
        // Run random_board until we find a non-winning one
        while board.get_winner().is_some() {
            board = random_board(&mut rng);
        }
        board
    }

    /// Return the winning player (if there is one)
    pub fn get_winner(&self) -> Option<Player> {
        // Checks if three squares are occupied and equal
        let check_win =
            |a: (usize, usize), b: (usize, usize), c: (usize, usize)| -> Option<Player> {
                if self[a] == self[b] && self[a] == self[c] && self[a] != Square::Empty {
                    Some(self[a].into())
                } else {
                    None
                }
            };
        let checks = [
            ((0, 0), (0, 1), (0, 2)),
            ((1, 0), (1, 1), (1, 2)),
            ((2, 0), (2, 1), (2, 2)),
            ((0, 0), (1, 0), (2, 0)),
            ((0, 1), (1, 1), (2, 1)),
            ((0, 2), (1, 2), (2, 2)),
            ((0, 0), (1, 1), (2, 2)),
            ((0, 2), (1, 1), (2, 0)),
        ];
        for check in checks {
            if let Some(winner) = check_win(check.0, check.1, check.2) {
                return Some(winner);
            }
        }
        None
    }

    /// Returns the unoccupied squares on the board
    pub fn get_unoccupied(&self) -> Vec<(usize, usize)> {
        let mut unoccupied = Vec::new();
        for x in 0..3 {
            for y in 0..3 {
                if self.0[x][y] == Square::Empty {
                    unoccupied.push((x, y));
                }
            }
        }
        unoccupied
    }
}
impl std::ops::Index<(usize, usize)> for Board {
    type Output = Square;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.0[x][y]
    }
}
impl std::ops::IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.0[x][y]
    }
}
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ys in self.0 {
            for y in ys {
                match y {
                    Square::Occupied(Player::X) => write!(f, "X")?,
                    Square::Occupied(Player::O) => write!(f, "O")?,
                    Square::Empty => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
