use anyhow::{Context, bail};
use std::path::PathBuf;
use std::{env, fs};

pub struct Config {
    pub dir: Dir,
    pub file: File,
    pub server: Server,
}

pub struct Dir {
    pub workdir: PathBuf,
    pub temp: PathBuf,
    pub dict: PathBuf,
    pub db: PathBuf,
}

pub struct File {
    pub db: PathBuf,
}

pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn new(workdir: Option<String>, host: String, port: u16) -> anyhow::Result<Self> {
        let current_exe_dir = env::current_exe()?
            .parent()
            .context("Failed to get parent dir of current_exe")?
            .to_path_buf();
        let workdir = workdir.map_or(current_exe_dir, PathBuf::from);

        let dir = Dir {
            workdir: workdir.clone(),
            temp: workdir.join("temp"),
            dict: workdir.join("dict"),
            db: workdir.join("db"),
        };
        if !dir.workdir.exists() {
            bail!("Workdir does not exist: {:?}", dir.workdir);
        }

        fs::create_dir_all(&dir.dict).context("Failed to create dict dir")?;
        fs::create_dir_all(&dir.temp).context("Failed to create temp dir")?;
        fs::create_dir_all(&dir.db).context("Failed to create db dir")?;

        let file = File {
            db: dir.db.join("db.sqlite"),
        };
        let server = Server { host, port };
        let config = Config { dir, file, server };
        Ok(config)
    }
}
