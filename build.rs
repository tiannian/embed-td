use std::{env, fs, path::Path, process::Command};

use flate2::read::GzDecoder;
use tar::Archive;

fn check_run(p: &Path, version: Option<&str>) -> bool {
    let output = Command::new(p).arg("version").output().unwrap();

    if let Some(v) = version {
        let ver = String::from_utf8(output.stdout).unwrap();
        ver.trim() == v.trim()
    } else {
        output.status.success()
    }
}

// None: No this dir
// Some(true): tendermint right
// Some(false): tendermint error
fn check_has_tendermint(dir: &str, version: Option<&str>) -> Option<bool> {
    let out_path = Path::new(dir).join("tendermint");

    if out_path.exists() {
        Some(check_run(&out_path, version))
    } else {
        None
    }
}

fn download_unpack_tgz(url: &str, out_dir: &str) {
    let body = reqwest::blocking::get(url).unwrap();
    let decoder = GzDecoder::new(body);
    let mut archive = Archive::new(decoder);
    println!("ssss");
    archive.unpack(out_dir).unwrap();
}

fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let sys = env::var("CARGO_CFG_TARGET_OS").unwrap();

    let platform = match (sys.as_str(), arch.as_str()) {
        ("linux", "aarch64") => "linux_arm64",
        ("linux", "arm") => "linux_armv6",
        ("linux", "x86_64") => "linux_amd64",
        ("android", "aarch64") => "linux_arm64",
        ("android", "arm") => "linux_armv6",
        ("android", "x86_64") => "linux_amd64",
        ("windows", "aarch64") => "windows_arm64",
        ("windows", "arm") => "windows_armv6",
        ("windows", "x86_64") => "windows_amd64",
        ("darwin", "aarch64") => "darwin_arm64",
        ("darwin", "x86_64") => "darwin_amd64",
        _ => {
            let target_triple = env::var("TARGET").unwrap();

            panic!("unsupport triple: {}", target_triple);
        }
    };

    let version = {
        if env::var("CARGO_FEATURE_TD_VER_0_34").unwrap() == "1" {
            "0.34.24"
        } else {
            panic!("must use special version of tendermint")
        }
    };

    let use_source_code = env::var("CARGO_FEATURE_USE_SOURCE_CODE");

    let upstream_url = env::var("EMBEDDED_TD_UPSTREAM_URL").ok();

    if use_source_code.is_err() {
        let dir = format!("{}/build", env::var("OUT_DIR").unwrap());

        if let Some(url) = upstream_url {
            if let Some(v) = check_has_tendermint(&dir, None) {
                if !v {
                    fs::remove_dir_all(&dir).unwrap();
                    download_unpack_tgz(&url, &dir);
                }
            } else {
                download_unpack_tgz(&url, &dir);
            }
        } else {
            let url = format!("https://github.com/tendermint/tendermint/releases/download/v{}/tendermint_{}_{}.tar.gz",version, version, platform);

            if let Some(v) = check_has_tendermint(&dir, Some(version)) {
                if !v {
                    fs::remove_dir_all(&dir).unwrap();
                    download_unpack_tgz(&url, &dir);
                }
            } else {
                download_unpack_tgz(&url, &dir);
            }
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=EMBEDDED_TD_UPSTREAM_URL");
}
