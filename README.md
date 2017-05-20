# cargo_toml_validate

This is a crate which helps to validate if all mandatory fields in a `Cargo.toml` file are present.

## Usage

```rust

let cargo_toml = "...";

match cargo_toml_validate::validate(cargo_toml) {
    Ok(()) => { /* Everything's fine */ },
    Err(errors) => {
        for error in errors {
          println!("{:?}", error);
        }
    }
}

```

It validates the following fields:

- description
- license
- license-file
- homepage
- repository


## License

MIT