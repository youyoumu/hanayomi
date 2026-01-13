use std::env;

use anyhow::Context;

pub fn parse_dict(workdir: Option<String>) -> anyhow::Result<()> {
    let current_exe_dir = env::current_exe()?
        .parent()
        .context("Failed to get parent dir of current_exe")?
        .to_string_lossy()
        .to_string();
    let workdir = workdir.unwrap_or(current_exe_dir);
    eprintln!("DEBUG[1410]: workdir={:#?}", workdir);
    Ok(())
}
