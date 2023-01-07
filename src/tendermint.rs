use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::{Child, Command},
};

use async_abci::ServerXX;
use rust_embed::RustEmbed;
use serde::Serialize;
use tempfile::tempdir;

use crate::{crypto::Keypair, defined, model, App, Config, Error, Genesis, Result, signal::stop_process};

#[derive(RustEmbed)]
#[folder = "$OUT_DIR/build"]
#[include = "tendermint"]
pub(crate) struct TendermintEmbed;

/// Tendermint instance
#[derive(Debug)]
pub struct Tendermint {
    #[cfg(not(feature = "__debug_tmp"))]
    work_dir: tempfile::TempDir,

    #[cfg(feature = "__debug_tmp")]
    work_dir: PathBuf,

    tendermint_child: Option<Child>,
}

impl Tendermint {
    pub fn get_binary_path(&self) -> PathBuf {
        let path = self.get_work_dir();

        path.join(defined::TENDERMINT_BIN_FILE)
    }

    pub fn get_config_dir(&self) -> PathBuf {
        let path = self.get_work_dir();

        path.join(defined::CONFIG_DIR)
    }

    pub fn get_config_path(&self) -> PathBuf {
        let path = self.get_work_dir();

        path.join(defined::CONFIG_FILE)
    }

    pub fn get_node_key_path(&self) -> PathBuf {
        let path = self.get_work_dir();

        path.join(defined::NODE_KEY_FILE)
    }

    pub fn get_validator_key_path(&self) -> PathBuf {
        let path = self.get_work_dir();

        path.join(defined::VALIDATOR_KEY_FILE)
    }

    pub fn get_socket_dir(&self) -> PathBuf {
        let path = self.get_work_dir();

        path.join(defined::SOCKET_DIR)
    }

    pub fn get_p2p_dir(&self) -> PathBuf {
        let path = self.get_work_dir();

        path.join(defined::P2P_DIR)
    }

    pub(crate) fn get_work_dir(&self) -> &Path {
        self.work_dir.as_ref()
    }

    pub fn get_app_path(&self) -> PathBuf {
        self.get_work_dir().join(defined::APP_UNIX_SOCKET_FILE)
    }
}

impl Tendermint {
    pub fn new() -> Result<Self> {
        #[cfg(feature = "__debug_tmp")]
        let this = {
            let work_dir = tempdir()?.into_path();
            log::info!("Config dir is: {:?}", work_dir.to_str());
            Self {
                work_dir,
                tendermint_child: None,
            }
        };

        #[cfg(not(feature = "__debug_tmp"))]
        let this = {
            let work_dir = tempdir()?;

            Self {
                work_dir,
                tendermint_child: None,
            }
        };

        let ef = TendermintEmbed::get("tendermint").ok_or(Error::NoTendermint)?;

        let bin_path = this.get_binary_path();

        let mut binary_tempfile = File::create(&bin_path)?;

        binary_tempfile.write_all(&ef.data)?;

        #[cfg(unix)]
        {
            use std::{
                fs::{metadata, set_permissions},
                os::unix::fs::PermissionsExt,
            };

            let mut permission = metadata(&bin_path)?.permissions();
            permission.set_mode(0o755);
            set_permissions(&bin_path, permission)?;
        }

        fs::create_dir_all(this.get_config_dir())?;
        fs::create_dir_all(this.get_p2p_dir())?;
        fs::create_dir_all(this.get_socket_dir())?;

        Ok(this)
    }

    pub fn version(&self) -> Result<String> {
        let version = Command::new(self.get_binary_path())
            .arg("version")
            .output()?;

        let s = String::from_utf8(version.stdout)?;

        Ok(String::from(s.trim()))
    }

    fn prepare_start(
        &mut self,
        config: Config,
        node_key: Keypair,
        validator_key: Keypair,
        genesis: Genesis<impl Serialize>,
    ) -> Result<()> {
        if config.data_dir.is_empty() {
            fs::create_dir_all(self.get_work_dir().join(defined::DATA_DIR))?;
        }

        let mut file = File::create(self.get_config_path())?;
        let cm = config.into_model(self.get_work_dir().to_str().ok_or(Error::PathUtf8Error)?);
        let cs = toml::to_string_pretty(&cm)?;
        file.write_all(&cs.into_bytes())?;

        let mut file = File::create(self.get_node_key_path())?;
        let m = node_key.into_model();
        let cs = serde_json::to_string_pretty(&m)?;
        file.write_all(&cs.into_bytes())?;

        let mut file = File::create(self.get_validator_key_path())?;
        let m = validator_key.into_model();
        let cs = serde_json::to_string_pretty(&m)?;
        file.write_all(&cs.into_bytes())?;

        let mut file = File::create(&cm.genesis_file)?;
        let m = genesis.into_model();
        let cs = serde_json::to_string_pretty(&m)?;
        file.write_all(&cs.into_bytes())?;

        let validator_state = model::ValidatorState::default();
        let mut file = File::create(cm.priv_validator_state_file)?;
        let m = validator_state.into_model();
        let cs = serde_json::to_string_pretty(&m)?;
        file.write_all(&cs.into_bytes())?;

        let command = Command::new(self.get_binary_path())
            .arg("--home")
            .arg(self.get_work_dir())
            .arg("node")
            .spawn()?;

        self.tendermint_child = Some(command);

        Ok(())
    }

    /// Start tendermint
    ///
    /// Pass ABCI, Config, NodeKey, ValidatorKey, Genesis
    #[cfg(feature = "internal-abci")]
    pub fn start<A>(
        &mut self,
        config: Config,
        node_key: Keypair,
        validator_key: Keypair,
        app: A,
        genesis: Genesis<A::AppState>,
    ) -> Result<()>
    where
        A: App + Clone + 'static,
    {
        self.prepare_start(config, node_key, validator_key, genesis)?;

        let app_path = self.get_app_path();

        std::thread::spawn(move || {
            #[cfg(feature = "async-smol")]
            smol::block_on(async move {
                ServerXX::new(app)
                    .bind_unix(app_path)
                    .await
                    .unwrap()
                    .run()
                    .await
                    .unwrap();
            });
            #[cfg(feature = "async-tokio")]
            tokio::block_on(async move {
                ServerXX::new(app)
                    .bind_unix(app_path)
                    .await
                    .unwrap()
                    .run()
                    .await
                    .unwrap();
            });
        });

        Ok(())
    }

    pub fn start_external_abci<A>(
        &mut self,
        config: Config,
        node_key: Keypair,
        validator_key: Keypair,
        genesis: Genesis<impl Serialize>,
    ) -> Result<()> {
        self.prepare_start(config, node_key, validator_key, genesis)
    }

    pub fn stop(&mut self) -> Result<()> {
        let child = self
            .tendermint_child
            .as_ref()
            .ok_or(Error::NoTendermintStart)?;

        stop_process(child)?;

        Ok(())
    }

    pub fn kill(&mut self) -> Result<()> {
        let child = self
            .tendermint_child
            .as_mut()
            .ok_or(Error::NoTendermintStart)?;

        child.kill()?;

        Ok(())
    }

    pub fn wait(&mut self) -> Result<()> {
        let child = self
            .tendermint_child
            .as_mut()
            .ok_or(Error::NoTendermintStart)?;

        child.wait()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;
    use serde::Serialize;

    use crate::{AlgorithmType, Config, Genesis, Keypair, Tendermint};

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_version() {
        let td = Tendermint::new().unwrap();
        assert_eq!(&td.version().unwrap(), "0.34.24")
    }

    #[derive(Debug, Serialize)]
    struct AppState {}

    #[test]
    fn test_start() {
        init();

        let rng = thread_rng();
        let validator_key = Keypair::generate(AlgorithmType::Ed25519, rng.clone());

        let node_key = Keypair::generate(AlgorithmType::Ed25519, rng);

        let genesis = Genesis::<()>::generate(validator_key.public_key.clone());

        let config = Config::default();

        let mut tendermint = Tendermint::new().unwrap();

        tendermint
            .start(config, node_key, validator_key, (), genesis)
            .unwrap();

        tendermint.stop().unwrap();
    }
}
