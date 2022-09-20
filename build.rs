use std::{collections::HashMap, env, fs, io::Read, path::Path, process::Command};

use flate2::read::GzDecoder;
use sha2::{digest::Output, Digest, Sha256};
use tar::Archive;

pub struct DigestReader<R, D> {
    reader: R,
    digester: D,
}

impl<R: Read, D: Digest + Clone> DigestReader<R, D> {
    pub fn new(reader: R, digester: D) -> Self {
        Self { reader, digester }
    }

    pub fn digest(&mut self) -> Output<D> {
        self.digester.clone().finalize()
    }
}

impl<R: Read, D: Digest> Read for DigestReader<R, D> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.reader.read(buf)?;

        println!("{}", len);

        self.digester.update(buf);

        Ok(len)
    }
}

fn check_and_download(td_name: &str, version: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();

    let out_file = format!("{}/tendermint", out_dir);

    let out_path = Path::new(&out_file);

    if out_path.exists() {
        let output = Command::new(out_path).arg("version").output().unwrap();
        if output.status.success() {
            return;
        } else {
            fs::remove_file(out_path).unwrap();
        }
    }
    download(td_name, version, &out_file);
}

/* fn build_hash_map() -> HashMap<String, String> { */
/* let mut hm = HashMap::new(); */
/*  */
/* hm.insert( */
/*     String::from("tendermint_0.34.21_darwin_arm64.tar.gz"), */
/*     String::from("4a37152f5bd93544ed485124b6f8c9684d80adba5096f6675aab7c2b01af6c64"), */
/* ); */
/* hm.insert( */
/*     String::from("tendermint_0.34.21_windows_amd64.tar.gz"), */
/*     String::from("587bb0f47e2721be9c202113597c6b2188e97fa1e36299d2ed821700f40c195a"), */
/* ); */
/* hm.insert( */
/*     String::from("tendermint_0.34.21_windows_arm64.tar.gz"), */
/*     String::from("593ab9cf5a2c45dfcec9061f52028f882e0761c85d5dac6a4e04c2d23f440902"), */
/* ); */
/* hm.insert( */
/*     String::from("tendermint_0.34.21_windows_armv6.tar.gz"), */
/*     String::from("936a9eeb74fd0029f29a5995bc3cd0252773b70036cba07304de3e74cb3970bc"), */
/* ); */
/* hm.insert( */
/*     String::from("tendermint_0.34.21_linux_arm64.tar.gz"), */
/*     String::from("9595eaa7743974bfa64912d0abedcf1fcd459ff16c73b4c2faa52e9f6f426378"), */
/* ); */
/* hm.insert( */
/*     String::from("tendermint_0.34.21_linux_amd64.tar.gz"), */
/*     String::from("c260c2db3e6faac711b767b095bb7c3c1afe51b624f97bfb675aa8063d1530c7"), */
/* ); */
/* hm.insert( */
/*     String::from("tendermint_0.34.21_linux_armv6.tar.gz"), */
/*     String::from("c6d77a8c5066bf3b337fa956ab038fec358bfb307bd6f5cdc11b7bda9642436e"), */
/* ); */
/* hm.insert( */
/*     String::from("tendermint_0.34.21_darwin_amd64.tar.gz"), */
/*     String::from("e2648f5bfc7ef173bd15114e40ebf9350ab64f3931faedb296501ba14c9143fc"), */
/* ); */
/*  */
/* hm */
/* } */

fn download(platform: &str, version: &str, target: &str) {
    let filename = format!("tendermint_{}_{}.tar.gz", version, platform);

    let url = format!(
        "https://github.com/tendermint/tendermint/releases/download/v{}/{}",
        version, filename
    );

    let body = reqwest::blocking::get(url).unwrap();

    let digester = DigestReader::new(body, Sha256::new());

    let decoder = GzDecoder::new(digester);

    let mut archive = Archive::new(decoder);

    let entries = archive.entries().unwrap();

    for entry in entries {
        let mut entry = entry.unwrap();
        let path = entry.path().unwrap();

        if path.to_str() == Some("tendermint") {
            entry.unpack(target).unwrap();
        }
    }

    /*     let hash_bytes = archive.into_inner().into_inner().digest(); */
    /*  */
    /* let map = build_hash_map(); */
    /*  */
    /* assert_eq!(Some(&hex::encode(hash_bytes)), map.get(&filename)); */
}

fn main() {
    let target_triple = env::var("TARGET").unwrap();

    let tts: Vec<&str> = target_triple.split("-").collect();

    let arch = tts[0];
    let sys = tts[2];

    let td_name = match (sys, arch) {
        ("linux", "aarch64") => "linux_arm64",
        ("linux", "arm") => "linux_armv6",
        ("linux", "armv7") => "linux_armv6",
        ("linux", "x86_64") => "linux_amd64",
        ("windows", "aarch64") => "windows_arm64",
        ("windows", "x86_64") => "windows_amd64",
        ("darwin", "aarch64") => "darwin_arm64",
        ("darwin", "x86_64") => "darwin_amd64",
        _ => panic!("unsupport triple: {}", target_triple),
    };

    let version = {
        if env::var("CARGO_FEATURE_TD_VER_0_34").unwrap() == "1" {
            "0.34.21"
        } else {
            panic!("must use special version of tendermint")
        }
    };

    check_and_download(td_name, version);
}
