<p align="center">
  <h1 align="center">
    The Encyclopedia<br />of Composable Characteristics
  </h1>
</p>

## 🖥️ Development

To bootstrap a development environment, please use the following commands.

```bash
# Clone the repository
git clone git@github.com:stjudecloud/ecc.git
cd ecc

# Build the crate in release mode
cargo build --release

# List out the examples
cargo run --release --example
```

## 🤖 Authoring characteristics with Claude

This repository ships a [Claude Code][claude-code] skill, `create-ecc`, that
guides an agent through **researching and authoring** a new composable
characteristic — not just writing YAML, but producing an evidence-backed entry a domain expert would sign off on. It lives at `.claude/skills/create-ecc/SKILL.md` and is discovered automatically by any agent started in this repository.

### What it does

1. **Research first.** Before writing anything, the agent builds an evidence
   brief: a precise definition and scope, the correct category (`morph` vs
   `molec`), primary-literature citations with resolvable URLs, how the property is measured/reported (which decides binary vs. enumerated values and any thresholds), and alignment with existing ontologies (NCIt/MONDO). A quality bar rejects thin or unsourced entries.
2. **Scaffold correctly.** It allocates the next identifier
   (`ECC-{MORPH|MOLEC}-NNNNNN`, six zero-padded digits) and a matching
   `NNNNNN-slug.yml` filename in the right folder, and writes a schema-valid entry
   honoring the `state` lifecycle (`draft` → `proposed` → `provisional` →
   `adopted`) and `deny_unknown_fields`.
3. **Validate.** It runs `cargo run -p ecc-cli -- check ecc` and fixes any
   failures before finishing.

### Usage

Start an agent in the repository root and describe the characteristic, e.g.:

> Use the create-ecc skill to add a molecular characteristic for MYCN
> amplification.

See [`.claude/skills/create-ecc/SKILL.md`](.claude/skills/create-ecc/SKILL.md)
for the full workflow and schema reference.

[claude-code]: https://www.anthropic.com/claude-code
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
