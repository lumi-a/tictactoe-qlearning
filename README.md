A super-basic implementation of [Q-learning](https://en.wikipedia.org/wiki/Q-learning) for TicTacToe in Rust, so that I improve my understanding of the lecture-material of a Reinforcement-Learning-class.

Run with `cargo run -r`. This first trains the quality-function-policy, and then plays a tournament against a random player. Results will be around:
```
Q-Learning: 98.95%
Random:     00.78%
Draws:      00.27%
```

I used Rust for speed and type-safety, though the code does look clunky for something as simple as TicTacToe. If rewritten, it probably should have some abstract game-traits (having states, transition-functions, rewards, etc.).