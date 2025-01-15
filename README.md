<p align="center">
  <h1 align="center">
    The Encyclopedia<br />of Composable Characteristics
  </h1>
</p>

## 🖥️ Development

To bootstrap a development environment, please use the following commands.

Foo bar baz.

```bash
# Clone the repository
git clone git@github.com:stjudecloud/ecc.git
cd ecc

# Build the crate in release mode
cargo build --release

# List out the examples
cargo run --release --example
```

## 🚧️ Tests

Before submitting any pull requests, please make sure the code passes the
following checks (from the root directory).

```bash
# Run the project's tests.
cargo test --all-features

# Run the tests for the examples.
cargo test --examples --all-features

# Ensure the project doesn't have any linting warnings.
cargo clippy --all-features

# Ensure the project passes `cargo fmt`.
# Currently this requires nightly Rust
cargo +nightly fmt --check

# Ensure the docs build.
cargo doc
```

## 🤝 Contributing

Contributions, issues and feature requests are welcome! Feel free to check
[issues page](https://github.com/stjudecloud/ecc/issues).

## 📝 License

This project is licensed as either [Apache 2.0][license-apache] or
[MIT][license-mit] at your discretion.

Copyright © 2024-Present [St. Jude Children's Research Hospital](https://github.com/stjude).

[license-apache]: https://github.com/stjudecloud/ecc/blob/main/LICENSE-APACHE
[license-mit]: https://github.com/stjudecloud/ecc/blob/main/LICENSE-MIT
