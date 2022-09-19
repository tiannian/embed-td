use std::{io::Write, process::Command};

use rust_embed::RustEmbed;
use tempfile::{NamedTempFile, TempPath};

use crate::{Error, Result};

#[derive(RustEmbed)]
#[folder = "$OUT_DIR"]
#[include = "tendermint"]
pub(crate) struct TendermintEmbed;

#[derive(Debug)]
pub struct Tendermint {
    binary_tempfile: TempPath,
}

impl Tendermint {
    pub fn new() -> Result<Self> {
        let mut binary_tempfile = NamedTempFile::new()?;

        let ef = TendermintEmbed::get("tendermint").ok_or(Error::NoTendermint)?;

        println!("len :{}", ef.data.len());

        binary_tempfile.write_all(&ef.data)?;

        let binary_tempfile = binary_tempfile.into_temp_path();

        #[cfg(unix)]
        {
            use std::{fs::{metadata, set_permissions}, os::unix::fs::PermissionsExt};

            let mut permission = metadata(&binary_tempfile)?.permissions();
            permission.set_mode(0o755);
            set_permissions(&binary_tempfile, permission)?;
        }

        Ok(Self {
            binary_tempfile,
        })
    }

    pub fn version(&self) -> Result<String> {
        let version = Command::new(&self.binary_tempfile).arg("version").output()?;

        println!("{:?}", version);

        let s = String::from_utf8(version.stdout)?;

        Ok(String::from(s.trim()))
    }

    /// Start tendermint
    ///
    /// Pass ABCI, Config, NodeKey, ValidatorKey, Genesis
    pub fn start(&self) -> Result<()> {
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
