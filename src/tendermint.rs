use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::Command,
};

use rust_embed::RustEmbed;
use tempfile::{tempdir, NamedTempFile, TempDir};

use crate::{defined, Config, Error, Keypair, Result};

#[derive(RustEmbed)]
#[folder = "$OUT_DIR"]
#[include = "tendermint"]
pub(crate) struct TendermintEmbed;

#[derive(Debug)]
pub struct Tendermint {
    work_dir: TempDir,
}

impl Tendermint {
    pub fn get_binary_path(&self) -> PathBuf {
        #[cfg(unix)]
        let bin_name = "tendermint";

        #[cfg(windows)]
        let bin_name = "tendermint.exe";

        self.work_dir.path().join(bin_name)
    }

    pub fn get_config_dir(&self) -> PathBuf {
        self.work_dir.path().join(defined::CONFIG_DIR)
    }

    pub fn get_config_path(&self) -> PathBuf {
        self.work_dir.path().join(defined::CONFIG_FILE)
    }

    pub fn get_node_key_path(&self) -> PathBuf {
        self.work_dir.path().join(defined::NODE_KEY_FILE)
    }

    pub fn get_validator_key_path(&self) -> PathBuf {
        self.work_dir.path().join(defined::VALIDATOR_KEY_FILE)
    }

    pub fn get_socket_dir(&self) -> PathBuf {
        self.work_dir.path().join(defined::SOCKET_DIR)
    }

    pub fn get_p2p_dir(&self) -> PathBuf {
        self.work_dir.path().join(defined::P2P_DIR)
    }
}

impl Tendermint {
    pub fn new() -> Result<Self> {
        let this = Self {
            work_dir: tempdir()?,
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
    pub fn start(&self, config: Config, node_key: Keypair, validator_key: Keypair) -> Result<()> {
        let config_file = {
            let mut config_file = NamedTempFile::new_in(self.get_config_path())?;
            let config_model = config
                .into_config_model(self.work_dir.path().to_str().ok_or(Error::PathUtf8Error)?);
            let cs = toml::to_string_pretty(&config_model)?;
            config_file.write_all(&cs.into_bytes())?;
            config_file.into_temp_path()
        };

        let node_key_file = {
            let mut node_key_file = NamedTempFile::new_in(self.get_node_key_path())?;
            let node_key_model = node_key.to_serde();
            let s = serde_json::to_vec_pretty(&node_key_model)?;
            node_key_file.write_all(&s)?;
            node_key_file.into_temp_path()
        };

        let validator_key_file = {
            let mut validator_key_file = NamedTempFile::new_in(self.get_validator_key_path())?;
            let m = validator_key.to_serde();
            let s = serde_json::to_vec_pretty(&m)?;
            validator_key_file.write_all(&s)?;
            validator_key_file.into_temp_path()
        };

        config_file.close()?;
        node_key_file.close()?;
        validator_key_file.close()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Tendermint;

    #[test]
    fn test_version() {
        let td = Tendermint::new().unwrap();
        assert_eq!(&td.version().unwrap(), "0.34.21")
    }
}
