extern crate toml;
use toml::Value;
use std::error::Error;

/// # Examples
///
///```
///use cargo_toml_validate;
///let cargo_toml = "...";
///
///match cargo_toml_validate::validate(cargo_toml) {
///    Ok(()) => { /* Everything's fine */ },
///    Err(errors) => {
///        for error in errors {
///            println!("{:?}", error);
///        }
///    }
///}
///```
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

    if package_section.get("license").is_none() && package_section.get("license-file").is_none() {
        errors.push("license or license-file is missing".into());
    }

    if package_section.get("homepage").is_none() && package_section.get("repository").is_none() {
        errors.push("homepage or repository are missing".into());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
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

    fn with_license_file() -> String {
"
[package]
name = \"cargo_toml_validate\"
version = \"2.0.0\"
authors = [\"Jan Schulte <hello@unexpected-co.de>\"]
license-file = \"license.md\"
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
        assert!(results.is_ok());
    }

    #[test]
    fn it_passes_with_license_file_instead_of_license() {
        let results = validate(&with_license_file());
        assert!(results.is_ok());
    }
}
