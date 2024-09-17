use ordered_float::OrderedFloat;
use rand::prelude::*;
use std::collections::HashMap;

use tictactoe::Board;

mod tictactoe;

fn qlearning_for_x() {
    type State = Board;
    type Action = (usize, usize);
    type Float = OrderedFloat<f64>;
    const NUM_TRIALS: usize = 1e7 as usize;

    let mut rng = rand::thread_rng();
    let mut q: HashMap<(State, Action), Float> = HashMap::new();

    for i in 0..NUM_TRIALS {
        let s: State = Board::random_x_board();
        let ε: f64 = 1.0 / (i as f64 + 1.0);
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
    }
}

fn main() {}
