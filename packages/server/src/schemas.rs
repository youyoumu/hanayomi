pub mod dictionary_term_bank_v3;

#[cfg(test)]
mod test {
    use super::dictionary_term_bank_v3::DictionaryTermBankV3;
    use serde_json::Value;
    use serde_json::json;
    use std::fs;

    fn get_term_bank() -> Vec<Value> {
        // Path is relative to the Cargo.toml of the package
        let data = fs::read_to_string("src/fixtures/term_bank.json").unwrap();
        let term_bank: Value = serde_json::from_str(&data).unwrap();
        let term_bank = term_bank.as_array().unwrap();
        term_bank.clone()
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
}
