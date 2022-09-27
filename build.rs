use std::{env, fs, path::Path, process::Command};

use flate2::read::GzDecoder;
use tar::Archive;

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

fn download(platform: &str, version: &str, target: &str) {
    let filename = format!("tendermint_{}_{}.tar.gz", version, platform);

    let url = format!(
        "https://github.com/tendermint/tendermint/releases/download/v{}/{}",
        version, filename
    );

    let body = reqwest::blocking::get(url).unwrap();

    // let digester = DigestReader::new(body, Sha256::new());

    let decoder = GzDecoder::new(body);

    let mut archive = Archive::new(decoder);

    let entries = archive.entries().unwrap();

    for entry in entries {
        let mut entry = entry.unwrap();
        let path = entry.path().unwrap();

        if path.to_str() == Some("tendermint") {
            entry.unpack(target).unwrap();
        }
    }
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
