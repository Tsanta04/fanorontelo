# Rust coded neural network for fanorontelo best move predictor
[![Rust](https://img.shields.io/badge/Rust-1.80-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust crate for experimenting with neural networks, games, and math utils. Dive into AI-driven gameplay like Fanorontelo (a malagasy game) with MinMax and NN magic!
The neural network will try to imitate the minimax algorithm for a fanorontelo bot which predict the best move from a given position

## Regard of this branch
Vectorization of batch processing using ndarray to increase performance

## Src directory Breakdown
- **neural.rs**: Neural network implementation
- **fanorona3.rs**: Game logic for fanorona 3x3 (fanorontelo)
- **tictactoe.rs**: [test] Game logic for a tictactoe game (to test mini generic modeling)
- **dataset.rs**: [test] Tictactoe dataset generation (to test mini generic modeling)
- **main.rs**: calling model creation and training or playing against a model

## Quick Start
# to play with a model (saved in <model_bin_path>)
```bash
cargo run play [options] <model_bin_path>
```
# create or train models (saving each epoch parameters in the model_save_dir directory)
```bash
cargo run cm [options] <model_save_dir>
```
