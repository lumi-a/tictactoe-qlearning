use ordered_float::OrderedFloat;
use rand::prelude::*;
use std::collections::HashMap;

use tictactoe::*;

mod tictactoe;

type State = Board;
type Action = (usize, usize);
type Float = OrderedFloat<f64>;
fn qlearning_for_x() -> HashMap<(State, Action), Float> {
    fn reward(winner: Player) -> Float {
        Float::from(match winner {
            Player::X => 1.0,
            Player::O => -1.0,
        })
    }
    const NUM_TRIALS: usize = 1e6 as usize;
    let gamma = Float::from(0.99);

    let mut rng = rand::thread_rng();
    let mut q: HashMap<(State, Action), Float> = HashMap::new();

    for i in 0..NUM_TRIALS {
        let s: State = Board::random_nonterminal_x_board();
        let epsilon: f64 = 1.0 / f64::sqrt(i as f64 + 1.0);
        let alpha: Float = Float::from(epsilon);
        // Sample a ε-greedily
        let possible_actions = s.get_unoccupied();
        let a: Action = if rng.gen_bool(epsilon) {
            *possible_actions.iter().choose(&mut rng).unwrap()
        } else {
            *possible_actions
                .iter()
                .max_by_key(|a| {
                    q.entry((s.clone(), **a))
                        .or_insert(Float::from(0.0))
                        .clone()
                })
                .unwrap()
        };

        let (next_s, r) = {
            let mut next = s.clone();
            next[a] = Square::Occupied(Player::X);

            // Did X already win?
            if let Some(winner) = next.get_winner() {
                (next, reward(winner))
            } else {
                let next_possible_actions = next.get_unoccupied();
                if !next_possible_actions.is_empty() {
                    next[*next_possible_actions.choose(&mut rng).unwrap()] =
                        Square::Occupied(Player::O);
                    if let Some(winner) = next.get_winner() {
                        (next, reward(winner))
                    } else {
                        (next, Float::from(0.0))
                    }
                } else {
                    // Draw
                    (next, Float::from(0.0))
                }
            }
        };

        let maxi = {
            let possible_actions = next_s.get_unoccupied();
            possible_actions
                .iter()
                .map(|a| {
                    q.entry((next_s.clone(), *a))
                        .or_insert(Float::from(0.0))
                        .clone()
                })
                .max()
                .unwrap_or(Float::from(0))
        };
        let index = (s.clone(), a);
        if !q.contains_key(&index) {
            q.insert(index.clone(), Float::from(0.0));
        }
        let value = (Float::from(1.0) - alpha) * q[&index] + alpha * (r + gamma * maxi);
        q.insert(index, value);
    }
    q
}

fn _get_human_input(possible_actions: &Vec<Action>) -> Action {
    let mut input: String;
    loop {
        input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let coors: Vec<&str> = input.trim().split_whitespace().into_iter().collect();
        if coors.len() == 2 {
            let x = coors[0].parse::<usize>().unwrap();
            let y = coors[1].parse::<usize>().unwrap();
            if possible_actions.contains(&(x, y)) {
                return (x, y);
            }
        }
    }
}

fn play_orandom(q: HashMap<(State, Action), Float>) -> Option<Player> {
    let mut rng = rand::thread_rng();
    let mut board = Board::empty();
    let mut i = 0;
    while board.get_winner().is_none() && !board.get_unoccupied().is_empty() {
        if i % 2 == 0 {
            // Q-Learning
            let possible_actions = board.get_unoccupied();
            let a = *possible_actions
                .iter()
                .max_by_key(|a| {
                    q.get(&(board.clone(), **a))
                        .cloned()
                        .unwrap_or(Float::from(0.0))
                })
                .unwrap();
            board[a] = Square::Occupied(Player::X);
        } else {
            // Random
            let possible_actions = board.get_unoccupied();
            let a = *possible_actions.choose(&mut rng).unwrap();
            board[a] = Square::Occupied(Player::O);
        }
        i += 1;
    }
    return board.get_winner();
}
fn main() {
    let q = qlearning_for_x();
    println!("Learned.");
    let mut q_wins = 0;
    let mut r_wins = 0;
    let mut draws = 0;
    const NUM_GAMES: usize = 1e4 as usize;
    for _ in 0..NUM_GAMES {
        match play_orandom(q.clone()) {
            Some(Player::X) => q_wins += 1,
            Some(Player::O) => r_wins += 1,
            None => draws += 1,
        }
    }
    println!(
        "Q-Learning: {}",
        (100.0 * (q_wins as f64) / (NUM_GAMES as f64)) as usize
    );
    println!(
        "Random:     {}",
        (100.0 * (r_wins as f64) / (NUM_GAMES as f64)) as usize
    );
    println!(
        "Draws:      {}",
        (100.0 * (draws as f64) / (NUM_GAMES as f64)) as usize
    );
}
