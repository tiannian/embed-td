use embeded_td::{example, AlgorithmType, Config, Genesis, Keypair, Tendermint};
use rand::thread_rng;

fn main() {
    env_logger::init();

    let rng = thread_rng();
    let validator_key = Keypair::generate(AlgorithmType::Ed25519, rng.clone());

    let node_key = Keypair::generate(AlgorithmType::Ed25519, rng);

    let genesis = Genesis::<example::ExampleAppState>::generate(validator_key.public_key.clone());

    let config = Config::default();

    let mut tendermint = Tendermint::new().unwrap();

    tendermint
        .start(config, node_key, validator_key, genesis)
        .unwrap();

    tendermint.wait().unwrap();
}
