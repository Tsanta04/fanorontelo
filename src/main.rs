#[allow(unused)]
use crate::{
    data::datasets::{balance_dataset_uniform, split_dataset, inspect_dataset},
    data::datasets::{generate_dataset, shuffle_dataset},
    games::fanorona::*,
    nn::NeuralNetwork,
    testing::train::{continue_train_model_with_batch},
    testing::{predict::test_model, train::{train_model_with_batch}},
};

mod data;
mod games;
mod maths;
mod nn;
mod testing;

fn main() {
    // TESTING MODELS
    // ==================== MODEL TESTING CONSTANTS ==========================
    // const TESTS_FILE: &str = "datasets/depth7/training.txt";
    // test_model("models/fn_model_d7_v7/fn_model_d7_v7_E100.bin", TESTS_FILE);



    // CREATING NEW MODELS
    // ==================== TRAINING CONSTANTS ===============================
    const LEARNING_RATE: f64 = 0.01;
    const EPOCHS: usize = 500;
    const STEP_SIZE: usize = EPOCHS/5;
    const BATCH_SIZE: usize = 2;
    const TRAIN_FILE: &str = "datasets/fanorona/all.txt";
    const VAL_FILE: &str = "datasets/fanorona/all.txt";

    const MODELS_DIR: &str = "models"; // Directory where models will be saved
    const MODEL_NAME: &str = "fn_model_dBatched_ng_v1"; // Each model will be saved for every epochs of the training
    const INPUT_SIZE: usize = 46;

    let layer_sizes: Vec<usize> = vec![64,64,18];
    let mut nn: NeuralNetwork = NeuralNetwork::new(&layer_sizes, INPUT_SIZE, LEARNING_RATE);
    train_model_with_batch(&mut nn, MODELS_DIR, TRAIN_FILE, VAL_FILE, MODEL_NAME, EPOCHS, BATCH_SIZE, STEP_SIZE);

    // // IF IT'S JUST AN UPGRADE OF A MODEL YOU CAN CONTINUE IT DOWN HERE BY LOADING THE MODEL
    // let EXISTENT_MODEL_NAME = "models/fn_model_dBatched_ng_v1/fn_model_dBatched_ng_v1_E100.bin";
    // let NEW_MODEL_NAME = "fn_model_dBatched_ng_v2";

    // continue_train_model_with_batch(
    //     EXISTENT_MODEL_NAME,
    //     NEW_MODEL_NAME,
    //     MODELS_DIR,
    //     TRAIN_FILE,
    //     VAL_FILE,
    //     EPOCHS,
    //     BATCH_SIZE,
    //     STEP_SIZE
    // );


    // =================================== DATASET GENERATION (with a depth as parameter)
    // let data_file: &str = "datasets/depth8/training.txt";
    // generate_dataset(7,data_file);
}
