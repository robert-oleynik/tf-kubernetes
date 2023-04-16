# Terraform Docker Kubernetes for Rust

[![crates](https://img.shields.io/crates/v/tf-kubernetes)](https://crates.io/crates/tf-kubernetes)

This repository contains the pre generated rust code for the Terraform Kubernetes
provider.

## Requirements

- `cargo`
- [`tf-bindgen`](https://github.com/robert-oleynik/tf-bindgen)
- [Terraform](https://www.terraform.io)

## Usage

Use the following commands we can add `tf-bindgen` and `tf-kubernetes` to your crate:

```sh
cargo add tf-bindgen
cargo add tf-kubernetes
```

Both crates are required to use this provider. See the [Documentation] of tf-bindgen
for more details.

[Documentation]: https://github.com/robert-oleynik/tf-bindgen/deployments/activity_log?environment=github-pages

## Contributing

<!-- TODO: add placeholder text -->

## License

This project is licensed under the [BSD-3-Clause](./LICENSE) license.
