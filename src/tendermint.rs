use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::{Child, Command},
};

use rust_embed::RustEmbed;
use serde::Serialize;
use tempfile::tempdir;

use crate::{defined, model, Config, Error, Genesis, Keypair, Result};

#[derive(RustEmbed)]
#[folder = "$OUT_DIR"]
#[include = "tendermint"]
pub(crate) struct TendermintEmbed;

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

    pub fn get_data_dir(&self) -> PathBuf {
        let path = self.get_work_dir();

        path.join(defined::DATA_DIR)
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

    pub fn get_genesis_path(&self) -> PathBuf {
        let path = self.get_work_dir();

        path.join(defined::GENESIS_FILE)
    }

    pub fn get_validator_state_path(&self) -> PathBuf {
        let path = self.get_work_dir();

        path.join(defined::VALIDATOR_STATE_FILE)
    }

    pub(crate) fn get_work_dir(&self) -> &Path {
        self.work_dir.as_ref()
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
        fs::create_dir_all(this.get_data_dir())?;

        Ok(this)
    }

    pub fn version(&self) -> Result<String> {
        let version = Command::new(&self.get_binary_path())
            .arg("version")
            .output()?;

        println!("{:?}", version);

        let s = String::from_utf8(version.stdout)?;

        Ok(String::from(s.trim()))
    }

    /// Start tendermint
    ///
    /// Pass ABCI, Config, NodeKey, ValidatorKey, Genesis
    pub fn start<AppState>(
        &mut self,
        config: Config,
        node_key: Keypair,
        validator_key: Keypair,
        genesis: Genesis<AppState>,
    ) -> Result<()>
    where
        AppState: Serialize,
    {
        macro_rules! create_file {
            ($arg:expr, $func_name:ident, $type_fn:path) => {{
                let mut file = File::create(self.$func_name())?;
                let m = $arg.into_model();
                let cs = $type_fn(&m)?;
                file.write_all(&cs.into_bytes())?;
            }};
        }

        {
            let mut file = File::create(self.get_config_path())?;
            let m = config.into_model(self.get_work_dir().to_str().ok_or(Error::PathUtf8Error)?);
            let cs = toml::to_string_pretty(&m)?;
            file.write_all(&cs.into_bytes())?;
        }

        create_file!(node_key, get_node_key_path, serde_json::to_string_pretty);
        create_file!(
            validator_key,
            get_validator_key_path,
            serde_json::to_string_pretty
        );
        create_file!(genesis, get_genesis_path, serde_json::to_string_pretty);

        let validator_state = model::ValidatorState::default();
        create_file!(validator_state, get_validator_key_path, serde_json::to_string_pretty);

        let command = Command::new(self.get_binary_path())
            .arg("--home")
            .arg(self.get_work_dir())
            .arg("start")
            .spawn()?;

        self.tendermint_child = Some(command);

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        // TODO: Use sigint.
        let child = self
            .tendermint_child
            .as_mut()
            .ok_or(Error::NoTendermintStart)?;

        child.kill()?;

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

    use crate::{example, AlgorithmType, Config, Genesis, Keypair, Tendermint};

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_version() {
        let td = Tendermint::new().unwrap();
        assert_eq!(&td.version().unwrap(), "0.34.21")
    }

    #[derive(Debug, Serialize)]
    struct AppState {}

    #[test]
    fn test_start() {
        init();

        let rng = thread_rng();
        let validator_key = Keypair::generate(AlgorithmType::Ed25519, rng.clone());

        let node_key = Keypair::generate(AlgorithmType::Ed25519, rng);

        let genesis =
            Genesis::<example::ExampleAppState>::generate(validator_key.public_key.clone());

        let config = Config::default();

        let mut tendermint = Tendermint::new().unwrap();

        tendermint
            .start(config, node_key, validator_key, genesis)
            .unwrap();
    }
}
