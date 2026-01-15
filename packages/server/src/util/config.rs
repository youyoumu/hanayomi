use anyhow::{Context, bail};
use std::path::PathBuf;
use std::sync::OnceLock;
use std::{env, fs};

struct Config {
    dir: Dir,
}

pub struct Dir {
    pub workdir: PathBuf,
    pub temp: PathBuf,
    pub dict: PathBuf,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init_config(workdir: Option<String>) -> anyhow::Result<()> {
    let current_exe_dir = env::current_exe()?
        .parent()
        .context("Failed to get parent dir of current_exe")?
        .to_path_buf();
    let workdir = workdir.map_or(current_exe_dir, PathBuf::from);

    let dir = Dir {
        workdir: workdir.clone(),
        temp: workdir.join("temp"),
        dict: workdir.join("dict"),
    };
    if !dir.workdir.exists() {
        bail!("Workdir does not exist: {:?}", dir.workdir);
    }

    fs::create_dir_all(&dir.dict).context("Failed to create dict dir")?;
    fs::create_dir_all(&dir.temp).context("Failed to create temp dir")?;
    let config = Config { dir };
    if CONFIG.set(config).is_err() {
        bail!("Error: Config was already initialized!");
    }

    Ok(())
}

pub fn get_dir() -> anyhow::Result<&'static Dir> {
    let config = CONFIG.get().context("Failed to get config")?;
    Ok(&config.dir)
}
