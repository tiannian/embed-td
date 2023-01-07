use std::process::Child;

use nix::{unistd::Pid, sys::signal::{kill, Signal}};

use crate::Result;

pub fn stop_process(child: &Child) -> Result<()> {
    let pid = child.id();

    let p = Pid::from_raw(pid as i32);
    kill(p, Signal::SIGINT)?;

    Ok(())
}
