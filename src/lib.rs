extern crate toml;
use toml::Value;
use std::error::Error;

pub fn validate(cargo_toml: &str) -> Result<(), Vec<String>> {
    let toml_table = match cargo_toml.parse::<Value>() {
        Ok(table) => table,
        Err(err) => return Err(vec!(err.description().into()))
    };

    let package_section = toml_table["package"].as_table().unwrap();

    let mut errors = vec!();

    if package_section.get("description").is_none() {
        errors.push("Description is missing".into());
    }

    if package_section.get("license").is_none() {
        errors.push("license or license-file is missing".into());
    }

    if package_section.get("homepage").is_none() {
        if package_section.get("repository").is_none() {
            errors.push("homepage or repository are missing".into());
        }
    }

    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(())
    }
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


    fn description_is_missing() -> String {
"
[package]
name = \"cargo_toml_validate\"
version = \"2.0.0\"
authors = [\"Jan Schulte <hello@unexpected-co.de>\"]
license = \"MIT\"
repository = \"https://github.com/schultyy/cargo_toml_validate\"
".into()
    }

    fn repository_is_missing() -> String {
"
[package]
name = \"cargo_toml_validate\"
version = \"2.0.0\"
authors = [\"Jan Schulte <hello@unexpected-co.de>\"]
license = \"MIT\"
description = \"This and that\"
".into()
    }

    fn license_is_missing() -> String {
"
[package]
name = \"cargo_toml_validate\"
version = \"2.0.0\"
authors = [\"Jan Schulte <hello@unexpected-co.de>\"]
repository = \"https://github.com/schultyy/cargo_toml_validate\"
description = \"This and that\"
".into()
    }

    fn valid_cargo_toml() -> String {
"
[package]
name = \"cargo_toml_validate\"
version = \"2.0.0\"
authors = [\"Jan Schulte <hello@unexpected-co.de>\"]
license = \"MIT\"
description = \"This and that\"
repository = \"https://github.com/schultyy/cargo_toml_validate\"
".into()
    }

    #[test]
    fn it_should_fail_with_errors() {
        let results = validate(&invalid_cargo_toml());
        assert!(results.is_err());
        assert_eq!(3, results.unwrap_err().len());
    }

    #[test]
    fn it_should_fail_when_cargo_toml_is_malformed() {
        let results = validate(&malformed_cargo_toml());
        assert!(results.is_err());
        assert_eq!(1, results.unwrap_err().len());
    }

    #[test]
    fn it_should_fail_when_description_is_missing() {
        let results = validate(&description_is_missing());
        assert!(results.is_err());
        assert_eq!(1, results.unwrap_err().len());
    }

    #[test]
    fn it_should_fail_when_license_is_missing() {
        let results = validate(&license_is_missing());
        assert!(results.is_err());
        println!("{:?}", results);
        assert_eq!(1, results.unwrap_err().len());
    }

    #[test]
    fn it_should_fail_when_repository_is_missing() {
        let results = validate(&repository_is_missing());
        assert!(results.is_err());
        assert_eq!(1, results.unwrap_err().len());
    }

    #[test]
    fn it_passes_with_valid_cargo_toml() {
        let results = validate(&valid_cargo_toml());
        println!("{:?}", results);
        assert!(results.is_ok());
    }
}
