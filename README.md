# Rust-Coded Neural Network for Fanoron-telo

[![Rust](https://img.shields.io/badge/Rust-1.80-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This project is a from-scratch implementation of a Neural Network in Rust, designed to play the Malagasy board game Fanoron-telo (a 3x3 variant of Fanorona). The AI is trained to predict the best possible move from any given board state by learning to imitate a powerful Minimax algorithm.

The entire neural network, including matrix operations, activation functions, and backpropagation, is built without external machine learning libraries.


## Team Presentation

This project was developed by a group of students passionate about Artificial Intelligence and Parallel Programming.
Each member contributed to different aspects of the implementation, from theoretical research to practical coding and testing.

-  RAMAROSON Tojotiana Cyriaque

-  RAKOTOARIJAONA Aro Mino Avotra

-  RANDRIANARISOA Tsantamirindra

-  RANAIVO Ny Aina Peniala


## Project Branches

This repository contains several branches, each representing a different stage or approach to the implementation:

-  `main`\ `ndarray_version`: An experimental version that replaces the custom matrix/vector implementation with the popular `ndarray` crate. The performance comparison is discussed in its dedicated LaTeX document.

-  `batch-training-dynamic-lr`: The primary branch. It features a robust implementation with **batch training** and a **hybrid dynamic learning rate scheduler**. Its theoretical foundations are detailed in an accompanying LaTeX report.

-   `stochastic`: A more fundamental implementation that uses **Stochastic Gradient Descent (SGD)**, where weights are updated after every single example. The mathematical principles are explained in the corresponding LaTeX report.

## Project Structure

- For hand coded branches
```
.
├── apk/                  # Android APK build for the game
├── Cargo.toml
├── dataset/              # Contains datasets for training
│   └── fanorona/
├── latex/                # LaTeX source files for project reports
├── models/               # Directory where trained models are saved (.gitignore'd)
├── src/
│   ├── main.rs           # Main entry point (CLI)
│   ├── data/             # Data loading and generation
│   ├── games/            # Game logic (Fanorona, Minimax)
│   ├── maths/            # Custom math library (Vector, Matrix)
│   ├── nn/               # Neural network implementation
│   └── testing/          # Training and evaluation scripts
└── README.md
```
- For ndarray / main version
    - **neural.rs**: Neural network implementation
    - **fanorona3.rs**: Game logic for fanorona 3x3 (fanorontelo)
    - **tictactoe.rs**: [test] Game logic for a tictactoe game (to test mini generic modeling)
    - **dataset.rs**: [test] Tictactoe dataset generation (to test mini generic modeling)
    - **main.rs**: calling model creation and training or playing against a model

## Core Features

1.  **Data Generation**: The project can generate its own high-quality training data by using a Minimax algorithm to explore all possible game states and determine the optimal move for each.
2.  **Neural Network Training**: A fully-featured training loop trains the network to predict the best move (both the starting and ending square) from a one-hot encoded board state.
3.  **Model Persistence**: Trained models (weights and biases) can be saved to and loaded from binary files, allowing for training to be paused and resumed, or for a pre-trained model to be used for gameplay.
4.  **Performance Evaluation**: The training script continuously evaluates the model's accuracy and loss on both training and validation datasets.

### Advanced Training Features

#### 1. Batch Training

To stabilize and accelerate the learning process, the model is trained using mini-batches. Instead of updating the network's weights after every single example, the gradients (the "corrections" for the weights) are calculated for each position in a batch, averaged together, and then a single update is applied. This provides a more stable and generalized update direction. The batch size is configurable via the `batch_size` constant in `main.rs`.

#### 2. Hybrid Learning Rate Scheduling

The learning rate (`lr`) is a critical hyperparameter that controls the size of weight updates. This project implements a sophisticated hybrid approach to adjust it dynamically during training:

-   **Step Decay**: The learning rate is scheduled to decrease exponentially after a set number of epochs (`step_size`). It starts high to allow for rapid initial convergence and then shrinks to enable finer, more precise adjustments as the model gets closer to a solution. The formula is `lr = initial_lr * 0.5^(epoch / step_size)`.

-   **ReduceLROnPlateau**: This is a reactive strategy. If the model's accuracy on the validation set fails to improve for a certain number of epochs (`patience`), the learning rate is immediately reduced by a `plateau_factor`. This helps the model escape local minima or performance plateaus by taking smaller, more careful steps.

These two techniques are combined in `testing/train.rs` to create a robust and adaptive learning schedule, which is key to achieving high model accuracy.



## How to Use

### Prerequisites

-   The Rust toolchain must be installed.

### 1. Prepare the Dataset

The model needs a dataset of `(board_state, best_move)` pairs.

**Option A: Generate a Dataset**

You can generate a complete dataset using the built-in Minimax engine.
1.  In `src/data/datasets.rs`, the `generate_dataset` function is available.
2.  You can call this function from `main.rs` to create a new data file.

    ```rust
    // Example call in main.rs
    // dataset::generate_dataset(7, "dataset/fanorona/depth7.txt");
    ```

3.  Run the program and redirect the output: `cargo run > dataset/fanorona/depth7.txt`.

**Option B: Use an Existing Dataset**

Ensure your dataset file (e.g., `all.txt`) is placed in the `dataset/fanorona/` directory. The project includes functions to split this data into training, validation, and test sets.

### 2. Train a Model

The `main.rs` file uses a command-line interface. To train a model, use the `cm` (create model) command.

1.  **Configure Training**: In `src/main.rs`, inside the `create_model` function, adjust the training constants:
    -   `lr`: Initial learning rate.
    -   `epochs`: Total number of training epochs.
    -   `layers`: The architecture of the neural network (e.g., `vec![512, output_size]`).
    -   `batch_size`: The number of samples per batch.
    -   `tr_file`: Path to your training data.

2.  **Start Training**: Run the following command. The argument is the directory where model files will be saved. The `--release` flag is highly recommended for a significant performance boost.

    ```bash
    # This will save models inside the "models/fanorona_v1" directory
    cargo run --release cm models/fanorona_v1
    ```

    A `.bin` file for the model will be saved in the specified directory after each epoch.

### 3. Continue Training an Existing Model

1.  In `src/main.rs` inside `create_model`, comment out the line for creating a new model:
    ```rust
    // let mut ne = Neural::xavier(layers, input_size);
    ```
2.  Uncomment and modify the line for loading an existing model:
    ```rust
    let model = "models/fanorona_v1/epoch_50.bin"; // Path to your saved model
    let mut ne = Neural::load_from_bin(model).unwrap();
    ```
3.  Run the training command again. It will pick up from where the loaded model left off.

### 4. Play Against the AI

Once you have a trained model (`.bin` file), you can play against it using the `play` command.

1.  Run the application with the `play` argument, followed by the path to your model file.

    ```bash
    cargo run --release play models/fanorona_v1/epoch_200.bin
    ```

2.  The game will start in your terminal. The board will be displayed, and you can enter your moves.

### 5. Testing the Android APK

The `apk/` directory contains a pre-built Android application to play the game.

1.  **Installation**: Transfer the `.apk` file from the `apk/` directory to your Android device and install it. You may need to enable "Install from unknown sources" in your device's security settings.
2.  **Internet Connection**: The app requires an active internet connection to communicate with the neural network model, which is deployed on a free hosting service.
3.  **Important Note**: The free hosting service puts the server to sleep when it's inactive. **The first AI move in a new game session can take up to 5 minutes** while the server wakes up. Subsequent moves will be significantly faster.
