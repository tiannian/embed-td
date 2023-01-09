use std::{sync::{Arc, RwLock}, process::exit};

use embedded_td::{AlgorithmType, Config, Genesis, Keypair, Tendermint};
use rand::thread_rng;

fn main() {
    env_logger::init();

    let rng = thread_rng();
    let validator_key = Keypair::generate(AlgorithmType::Ed25519, rng.clone());

    let node_key = Keypair::generate(AlgorithmType::Ed25519, rng);

    let genesis = Genesis::<()>::generate(validator_key.public_key.clone());

    let config = Config::default();

    let mut tendermint = Tendermint::new().unwrap();

    tendermint
        .start(config, node_key, validator_key, (), genesis)
        .unwrap();

    let tendermint = Arc::new(RwLock::new(tendermint));

    let td = tendermint.clone();

    ctrlc::set_handler(move || {
        println!("T!rig ctrl c");
        let mut guard = td.write().unwrap();
        guard.stop().unwrap();
        guard.wait().unwrap();
        exit(0);
    })
    .unwrap();

    tendermint.write().unwrap().wait().unwrap();
}
