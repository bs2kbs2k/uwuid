# Uwuid

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
![Crates.io](https://img.shields.io/crates/v/uwuid)
![Crates.io](https://img.shields.io/crates/l/uwuid)
![Crates.io](https://img.shields.io/crates/d/uwuid)

Like UUIDs or ULIDs, but better

## Install
Add the following to your Cargo.toml:
```toml
uwuid = "0.3.0"
```

## Usage
```rs
// Create a uwuid:
let id = uwuid::UwuId::new();
// Convert it to a string:
let foo = format!("{}", id);
// Parse a uwuid from a string:
let bar: uwuid::UwuId = foo.parse().unwrap();
```

## License
[MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE) © bs2k