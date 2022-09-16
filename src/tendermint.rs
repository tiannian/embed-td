use std::{path::PathBuf, fs, io::Write};

use rust_embed::RustEmbed;
use tempfile::NamedTempFile;

use crate::{Config, Error, Result};

#[derive(RustEmbed)]
#[folder = "$OUT_DIR"]
#[include = "tendermint"]
pub(crate) struct TendermintEmbed;

#[derive(Debug)]
pub struct Tendermint {
    binary_tempfile: NamedTempFile,
    config: Config,
}

impl Tendermint {
    pub fn new_with_config(config: Config) -> Result<Self> {
        let mut binary_tempfile = NamedTempFile::new()?;

        let ef = TendermintEmbed::get("tendermint").ok_or(Error::NoTendermint)?;

        binary_tempfile.write_all(&ef.data)?;

        Ok(Self {
            binary_tempfile,
            config,
        })
    }

    pub fn new() -> Result<Self> {
        Ok(Self::new_with_config(Config::default())?)
    }

    /// Init tendermint based on config
    pub fn init(&self) -> Result<()> {
        Ok(())
    }

    pub fn generate_validator(&self) -> Result<()> {
        Ok(())
    }

    pub fn generate_node_key(&self) -> Result<()> {
        Ok(())
    }

    pub fn generate_genesis(&self) -> Result<()> {
        Ok(())
    }
}
