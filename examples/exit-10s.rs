use std::{
    thread::{self, sleep},
    time::Duration,
};

use async_abci::ServerXX;
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
        .start(config, node_key, validator_key, genesis)
        .unwrap();

    let path = tendermint.get_app_path();

    thread::spawn(move || {
        smol::block_on(async move {
            let s = ServerXX::new(()).bind_unix(path).await.unwrap();
            s.run().await.unwrap();
        });
    });

    sleep(Duration::new(10, 0));

    tendermint.stop().unwrap();
    tendermint.wait().unwrap();
}
