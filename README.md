# Rust AI Playground
[![Rust](https://img.shields.io/badge/Rust-1.80-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust crate for experimenting with neural networks, games, and math utils. Dive into AI-driven gameplay like Fanorontelo (a malagasy game) with MinMax and NN magic!
The neural network will try to imitate the minimax algorithm for a fanorontelo bot which predict the best move from a given position

## Src directory Breakdown

- **data/**: Manages datasets creation and data loading pipelines.
- **games/**: Core game logic, including Fanorona rules and MinMax AI strategies.
- **maths/**: Math foundations – activations (ReLU, Sigmoid, Softmax) and collectors for matrices/vectors.
- **nn/**: Neural network essentials: initialization, learning algorithms, and prediction routines.
- **testing/**: Utilities for testing predictions and training workflows.

## Quick Start

```bash
cargo run
