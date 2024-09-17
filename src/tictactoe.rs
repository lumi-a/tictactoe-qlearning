use rand::prelude::*;

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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Square {
    Occupied(Player),
    Empty,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Board([[Square; 3]; 3]);
impl Board {
    pub fn empty() -> Board {
        Board([[Square::Empty; 3]; 3])
    }

    pub fn random_nonfull_board() -> Board {
        let mut rng = rand::thread_rng();
        let num_pieces = rng.gen_range(0..9);
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
        pieces.shuffle(&mut rng);
        let mut i = 0;
        for p in &pieces[0..num_pieces] {
            board.0[p.0][p.1] = if i % 2 == 0 {
                Square::Occupied(Player::X)
            } else {
                Square::Occupied(Player::O)
            };
            i += 1;
        }
        board
    }

    pub fn random_x_board() -> Board {
        let mut rng = rand::thread_rng();
        let num_pieces = rng.gen_range(0..5) * 2;
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
        pieces.shuffle(&mut rng);
        let mut i = 0;
        for p in &pieces[0..num_pieces] {
            board.0[p.0][p.1] = if i % 2 == 0 {
                Square::Occupied(Player::X)
            } else {
                Square::Occupied(Player::O)
            };
            i += 1;
        }
        board
    }

    pub fn get_current_player(&self) -> Player {
        let mut pieces = 0;
        for ys in self.0 {
            for y in ys {
                match y {
                    Square::Occupied(Player::X) => pieces += 1,
                    Square::Occupied(Player::O) => pieces -= 1,
                    Square::Empty => {}
                }
            }
        }
        if pieces > 0 {
            Player::O
        } else {
            Player::X
        }
    }

    pub fn get_winner(&self) -> Option<Player> {
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
