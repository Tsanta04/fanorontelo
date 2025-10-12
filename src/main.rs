#[allow(unused)]
use crate::{
    data::datasets::{balance_dataset_uniform, inspect_dataset},
    data::datasets::{generate_dataset, shuffle_dataset},
    games::fanorona::*,
    nn::NeuralNetwork,
    testing::train::continue_train_model,
    testing::{predict::test_model, train::train_model},
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
    const LEARNING_RATE: f64 = 0.001;
    const EPOCHS: usize = 100;
    const TRAIN_FILE: &str = "datasets/depth7/training.txt"; // The dataset
    const MODELS_DIR: &str = "models"; // Directory where models will be saved
    const MODEL_NAME: &str = "fn_md"; // Each model will be saved for every epochs of the training
    const INPUT_SIZE: usize = 46;
    
    let layer_sizes: Vec<usize> = vec![64, 18];
    let mut nn: NeuralNetwork = NeuralNetwork::new(&layer_sizes, INPUT_SIZE, LEARNING_RATE);
    train_model(&mut nn, MODELS_DIR, TRAIN_FILE, MODEL_NAME, EPOCHS);

    // // IF IT'S JUST AN UPGRADE OF A MODEL YOU CAN CONTINUE IT DOWN HERE BY LOADING THE MODEL

    // let EXISTENT_MODEL = "models/fn_model_d7_ng_v4/fn_model_d7_ng_v4_E100.bin";
    // let NEW_MODEL_NAME = "fn_model_d7_ng_v5";
    // continue_train_model(
    //     EXISTENT_MODEL,
    //     NEW_MODEL_NAME,
    //     MODELS_DIR,
    //     TRAIN_FILE,
    //     EPOCHS,
    // );



    // GENERATE DATASET
    // =================================== DATASET GENERATION (with a depth as parameter)
    // Redirect it into a file
    // generate_dataset(7);
}
