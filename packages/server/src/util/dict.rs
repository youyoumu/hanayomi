use std::env;
use std::fs;
use std::path;
use std::path::Path;
use zip::ZipArchive;

use anyhow::Context;

pub fn parse_dict(workdir: Option<String>, dict: String) -> anyhow::Result<()> {
    let current_exe_dir = env::current_exe()?
        .parent()
        .context("Failed to get parent dir of current_exe")?
        .to_string_lossy()
        .to_string();
    let workdir = workdir.unwrap_or(current_exe_dir);
    let dict_dir = path::Path::new(&workdir).join("dict");
    let temp_dir = path::Path::new(&workdir).join("temp");
    let dict_file_name = Path::new(&dict)
        .file_name()
        .context("Failed to get file name of dict")?;
    let dict_extract_path = temp_dir.join(dict_file_name);
    fs::create_dir_all(&dict_dir).context("Failed to create dict dir")?;
    fs::create_dir_all(&temp_dir).context("Failed to create temp dir")?;
    fs::create_dir_all(&dict_extract_path).context("Failed to create temp dir")?;
    let dict_file = fs::File::open(dict)?;
    let mut dict_archive = ZipArchive::new(dict_file)?;
    dict_archive.extract(&dict_extract_path)?;

    eprintln!("DEBUG[1410]: workdir={:#?}", workdir);
    Ok(())
}
