use super::*;
use serde_json::{Value, json};
use std::fs;

fn get_term_bank() -> Vec<Value> {
    // Path is relative to the Cargo.toml of the package
    let data = fs::read_to_string("src/fixtures/term_bank.json").unwrap();
    let term_bank: Value = serde_json::from_str(&data).unwrap();
    let term_bank = term_bank.as_array().unwrap();
    term_bank.to_vec()
}

#[test]
fn should_parse_term_bank_0() {
    let term = json!([get_term_bank().first().unwrap()]);
    let result: Result<DictionaryTermBankV3, _> = serde_json::from_value(term);
    assert!(result.is_ok());
}

#[test]
fn should_not_parse_term_bank_1() {
    let term = json!([get_term_bank().get(1).unwrap()]);
    let result: Result<DictionaryTermBankV3, _> = serde_json::from_value(term);
    assert!(result.is_err());
}

#[test]
fn should_parse_term_bank_2() {
    let term = json!([get_term_bank().get(2).unwrap()]);
    let result: Result<DictionaryTermBankV3, _> = serde_json::from_value(term);
    assert!(result.is_ok());
}

#[test]
fn should_parse_term_bank_3() {
    let term = json!([get_term_bank().get(3).unwrap()]);
    let result: Result<DictionaryTermBankV3, _> = serde_json::from_value(term);
    assert!(result.is_ok());
}

#[test]
fn should_not_parse_term_bank_4() {
    let term = json!([get_term_bank().get(4).unwrap()]);
    let result: Result<DictionaryTermBankV3, _> = serde_json::from_value(term);
    assert!(result.is_err());
}

#[test]
fn should_parse_term_bank_5() {
    let term = json!([get_term_bank().get(5).unwrap()]);
    let result: Result<DictionaryTermBankV3, _> = serde_json::from_value(term);
    assert!(result.is_ok());
}

#[test]
fn should_parse_term_bank_6() {
    let term = json!([get_term_bank().get(6).unwrap()]);
    let result: Result<DictionaryTermBankV3, _> = serde_json::from_value(term);
    assert!(result.is_ok());
}

#[test]
fn should_not_parse_term_bank_7() {
    let term = json!([get_term_bank().get(7).unwrap()]);
    let result: Result<DictionaryTermBankV3, _> = serde_json::from_value(term);
    assert!(result.is_err());
}

#[test]
fn should_parse_term_bank_8() {
    let term = json!([get_term_bank().get(8).unwrap()]);
    let result: Result<DictionaryTermBankV3, _> = serde_json::from_value(term);
    assert!(result.is_ok());
}

#[test]
fn should_not_parse_term_bank_9() {
    let term = json!([get_term_bank().get(9).unwrap()]);
    let result: Result<DictionaryTermBankV3, _> = serde_json::from_value(term);
    assert!(result.is_err());
}

#[test]
fn should_parse_term_bank_10() {
    let term = json!([get_term_bank().get(10).unwrap()]);
    let result: Result<DictionaryTermBankV3, _> = serde_json::from_value(term);
    assert!(result.is_ok());
}

#[test]
fn should_not_parse_term_bank_11() {
    let term = json!([get_term_bank().get(11).unwrap()]);
    let result: Result<DictionaryTermBankV3, _> = serde_json::from_value(term);
    assert!(result.is_err());
}

#[test]
fn should_parse_term_bank_12() {
    let term = json!([get_term_bank().get(12).unwrap()]);
    let result: Result<DictionaryTermBankV3, _> = serde_json::from_value(term);
    assert!(result.is_ok());
}

#[test]
fn should_parse_term_bank_13() {
    let term = json!([get_term_bank().get(13).unwrap()]);
    let result: Result<DictionaryTermBankV3, _> = serde_json::from_value(term);
    assert!(result.is_ok());
    let result = result.unwrap();
    let first = result.first().unwrap();
    let definition = first.5.first().unwrap();

    if let Definition::Detailed(definition) = definition
        && let DetailedDefinition::StructuredContent(sc) = definition.as_ref()
        && let StructuredContent::Object(obj) = sc.content.as_ref()
        && let StructuredContentObject::Img(img) = obj.as_ref()
    {
        assert_eq!(img.width.unwrap(), -1.0);
        assert!(img.validate().is_err());
    } else {
        panic!("Failed to parse term");
    }
}

#[test]
fn should_parse_term_bank_14() {
    let term = json!([get_term_bank().get(14).unwrap()]);
    let result: Result<DictionaryTermBankV3, _> = serde_json::from_value(term);
    assert!(result.is_ok());
    let result = result.unwrap();
    let first = result.first().unwrap();
    let definition = first.5.first().unwrap();

    if let Definition::Detailed(definition) = definition
        && let DetailedDefinition::StructuredContent(sc) = definition.as_ref()
        && let StructuredContent::Object(obj) = sc.content.as_ref()
        && let StructuredContentObject::Img(img) = obj.as_ref()
    {
        assert!(img.width.is_none());
        assert_eq!(img.image_rendering, "pixelated");
        assert!(img.validate().is_ok());
    } else {
        panic!("Failed to parse term");
    }
}
