# Rust Coded Neural Network for Fanorontelo Best Move Predictor

[![Rust](https://img.shields.io/badge/Rust-1.80-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust project for experimenting with neural networks, games, and math utils. Dive into AI-driven gameplay for Fanoron-telo (a Malagasy game) with MinMax and neural network magic! The neural network will try to imitate the Minimax algorithm for a Fanoron-telo bot which predicts the best move from a given position.

## `src` Directory Breakdown

- **`data/`**: Manages dataset creation and data loading pipelines.
- **`games/`**: Core game logic, including Fanorona rules and MinMax AI strategies.
- **`maths/`**: Math foundations – activation functions (ReLU, Sigmoid, Softmax) and collectors for matrices/vectors.
- **`nn/`**: Neural network essentials: initialization, learning algorithms (feed-forward, backpropagation), and prediction routines.
- **`testing/`**: Utilities for testing predictions and training workflows.

## Features

This project is capable of:
1.  **Generating training data**: Using a Minimax algorithm, the program can explore game positions to determine the best move, creating a high-quality dataset.
2.  **Training a neural network**: The neural network learns to predict the best move (from-square and to-square) from a given game state.
3.  **Saving and loading models**: The network's weights and biases are saved after each epoch, allowing training to be resumed or a pre-trained model to be used.
4.  **Evaluating performance**: The project includes scripts to measure the model's accuracy on test and validation data.

### Specific Features

#### 1. Batch Training

To stabilize and accelerate learning, the model is trained in batches. Instead of updating the network's weights after each example, the process is as follows:
- Gradients (the "corrections" for the weights) are calculated for each position in the batch.
- These gradients are averaged over the entire batch.
- The weights and biases are updated once with this average gradient.

This provides a more stable update direction, less sensitive to the specifics of a single example. The batch size is configurable via the `BATCH_SIZE` constant in `main.rs`.

#### 2. Learning Rate Scheduling

The learning rate (`lr`) is a crucial hyperparameter that controls the magnitude of weight updates. This project combines two strategies to adjust it dynamically during training:

-   **Step Decay**: The learning rate is reduced exponentially at each epoch. It starts high for rapid convergence and then decreases to fine-tune the weights. The formula used is `lr = initial_lr * 0.5^(epoch / step_size)`.

-   **ReduceLROnPlateau**: A reactive strategy. If the validation accuracy does not improve for a certain number of epochs (`patience`), the learning rate is reduced by a factor (`plateau_factor`). This helps the model escape performance plateaus.

## Quick Start

### Prerequisites

-   Rust must be installed on your machine.

### 1. Prepare the Data

Before training, you need a dataset.

a. **Generation (if needed)**: Uncomment the `DATASET GENERATION` section in `src/main.rs` and run `cargo run > dataset/fanorona/your_file.txt`.

b. **Use an existing dataset**: Ensure your data file (e.g., `all.txt`) is placed in the `dataset/fanorona/` directory.

### 2. Train a New Model

1.  Open `src/main.rs`.
2.  Configure the constants in the `TRAINING CONSTANTS` section (`LEARNING_RATE`, `EPOCHS`, `BATCH_SIZE`, etc.).
3.  Ensure the `train_model_with_batch` section is active.
4.  Run the training with Cargo (release mode is recommended for performance):

    ```bash
    cargo run --release
    ```

Models will be saved in the `models/YOUR_MODEL_NAME/` directory after each epoch.
