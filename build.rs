use std::env;

use flate2::read::GzDecoder;
use tar::Archive;

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

    let file_name = format!("https://github.com/tendermint/tendermint/releases/download/v0.34.21/tendermint_{}_{}.tar.gz", version, td_name);

    let out_dir = env::var("OUT_DIR").unwrap();

    let out_file = format!("{}/tendermint", out_dir);

    let body = reqwest::blocking::get(file_name).unwrap();

    let decoder = GzDecoder::new(body);

    let mut archive = Archive::new(decoder);

    let entries = archive.entries().unwrap();

    for entry in entries {
        let mut entry = entry.unwrap();
        let path = entry.path().unwrap();

        if path.to_str() == Some("tendermint") {
            entry.unpack(&out_file).unwrap();
        }
    }
}
