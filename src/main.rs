use ordered_float::OrderedFloat;
use rand::prelude::*;
use std::collections::HashMap;

use tictactoe::*;

mod tictactoe;

type State = Board;
type Action = (usize, usize);
type Float = OrderedFloat<f64>;
fn qlearning_for_x() -> HashMap<(State, Action), Float> {
    const NUM_TRIALS: usize = 1e6 as usize;
    let γ = Float::from(0.999);

    let mut rng = rand::thread_rng();
    let mut q: HashMap<(State, Action), Float> = HashMap::new();

    for i in 0..NUM_TRIALS {
        let s: State = Board::random_x_board();
        let ε: f64 = 1.0 / (i as f64 + 1.0);
        let α: Float = Float::from(ε);
        // Sample a ε-greedily
        let possible_actions = s.get_unoccupied();
        let a: Action = if rng.gen_bool(ε) {
            *possible_actions.iter().choose(&mut rng).unwrap()
        } else {
            *possible_actions
                .iter()
                .max_by_key(|a| {
                    q.entry((s.clone(), **a))
                        .or_insert(Float::from(random::<f64>()))
                        .clone()
                })
                .unwrap()
        };

        let next_s = {
            let mut next = s.clone();
            next[a] = Square::Occupied(Player::X);
            next
        };
        let r = Float::from(match next_s.get_winner() {
            Some(Player::X) => 1.0,
            Some(Player::O) => -1.0,
            _ => 0.0,
        });

        let maxi = {
            let possible_actions = next_s.get_unoccupied();
            possible_actions
                .iter()
                .map(|a| {
                    q.entry((next_s.clone(), *a))
                        .or_insert(Float::from(random::<f64>()))
                        .clone()
                })
                .max()
                .unwrap_or(Float::from(0))
        };
        let index = (s.clone(), a);
        if !q.contains_key(&index) {
            q.insert(index.clone(), Float::from(random::<f64>()));
        }
        let value = (Float::from(1.0) - α) * q[&index] + α * (r + γ * maxi);
        q.insert(index, value);
    }
    q
}

fn get_human_input(possible_actions: &Vec<Action>) -> Action {
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
fn main() {
    let mut board = Board::empty();
    let q = qlearning_for_x();
    let mut i = 0;
    while board.get_winner().is_none() && !board.get_unoccupied().is_empty() {
        if i % 2 == 0 {
            // Q-Learning
            let possible_actions = board.get_unoccupied();
            let a = *possible_actions
                .iter()
                .max_by_key(|a| q[&(board.clone(), **a)])
                .unwrap();
            board[a] = Square::Occupied(Player::X);
        } else {
            // Human
            let possible_actions = board.get_unoccupied();
            let a = get_human_input(&possible_actions);
            board[a] = Square::Occupied(Player::O);
        }
        i += 1;
        println!("{}\n\n", board);
    }
}
