extern crate toml;
use toml::Value;
use std::error::Error;

struct Key {
    pub name: String,
    pub optional_name: Option<String>
}

impl Key {
    pub fn new(name: String) -> Key {
        Key {
            name: name,
            optional_name: None
        }
    }

    pub fn new_with_optional_name(name: String, optional_name: String) -> Key {
        Key {
            name: name,
            optional_name: Some(optional_name)
        }
    }
}

pub fn validate(cargo_toml: &str) -> Result<(), Vec<String>> {
    let toml_table = match cargo_toml.parse::<Value>() {
        Ok(table) => table,
        Err(err) => return Err(vec!(err.description().into()))
    };

    let required_keys = vec!(Key::new("description".into()),
                            Key::new_with_optional_name("license".into(), "license-file".into()),
                            Key::new("documentation".into()),
                            Key::new_with_optional_name("homepage".into(), "repository".into()));
    Err(vec!("Description is missing".into(),
                "license or license-file is missing".into(),
                "documentation is missing".into(),
                "homepage or repository are missing".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn invalid_cargo_toml() -> String {
"
[package]
name = \"cargo_toml_validate\"
version = \"0.1.0\"
authors = [\"Jan Schulte <hello@unexpected-co.de>\"]

[dependencies]
".into()
    }

    fn malformed_cargo_toml() -> String {
"
[package]
name = \"cargo_toml_validate\"
version = \"0.1.0\"
autho hulte <hello@unexpected-co.de>\"]

[dependencies]
".into()
    }

    #[test]
    fn it_should_fail_with_errors() {
        let results = validate(&invalid_cargo_toml());
        assert!(results.is_err());
        assert_eq!(4, results.unwrap_err().len());
    }

    #[test]
    fn it_should_fail_when_cargo_toml_is_malformed() {
        let results = validate(&malformed_cargo_toml());
        assert!(results.is_err());
        assert_eq!(1, results.unwrap_err().len());
    }
}
