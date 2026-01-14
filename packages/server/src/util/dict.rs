use serde_json::json;
use std::env;
use std::fs;
use std::path;
use std::path::Path;
use zip::ZipArchive;

use anyhow::Context;

use crate::schemas::dictionary_term_bank_v3::DictionaryTermBankV3;

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
    // dict_archive.extract(&dict_extract_path)?;

    let term_bank_1 = fs::read_to_string(dict_extract_path.join("term_bank_1.json"))?;
    let terms: serde_json::Value = serde_json::from_str(&term_bank_1)?;
    let terms = terms.as_array().unwrap();
    let term1 = terms.get(100);
    println!("DEBUG[1410]: term1={:#?}", term1);
    let term1 = json!([term1]).to_string();

    let a: DictionaryTermBankV3 = serde_json::from_str(&term1)?;
    if let Some(term1) = a.first() {
        term1.test();
        // println!("DEBUG[1410]: term1={:#?}", term1);
    }

    eprintln!("DEBUG[1410]: workdir={:#?}", workdir);
    Ok(())
}
