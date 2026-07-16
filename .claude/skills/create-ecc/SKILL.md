---
name: create-ecc
description: Scaffold a new ECC (composable characteristic) as a schema-valid YAML file under ecc/morph or ecc/molec. Use when asked to add, create, or draft a new composable characteristic / ECC, or a new morphological or molecular characteristic. Produces a file that passes `cargo run -p ecc-cli -- check ecc`.
---

# Create an ECC (composable characteristic)

Scaffold a new composable characteristic as a YAML file that deserializes cleanly
into the `ecc::Characteristic` model. The authoritative schema lives in
`crates/ecc/src/` — this skill mirrors it. If in doubt, read `crates/ecc/src/lib.rs`,
`crates/ecc/src/common.rs`, `crates/ecc/src/common/value/kind.rs`, and
`crates/ecc/src/identifier.rs` before writing.

## 1. Pick the type, number, and identifier

Each ECC lives in one of two folders, and its identifier type must match:

| Type          | Folder       | Identifier prefix | Rust variant             |
| ------------- | ------------ | ----------------- | ------------------------ |
| Morphological | `ecc/morph/` | `ECC-MORPH-`      | `Identifier::Morphological` |
| Molecular     | `ecc/molec/` | `ECC-MOLEC-`      | `Identifier::Molecular`  |

- **Identifier format**: `ECC-{MORPH|MOLEC}-NNNNNN` — prefix and type are
  UPPERCASE, and the number is exactly **6 zero-padded digits**, starting at
  `000001` (zero is invalid). Example: `ECC-MOLEC-000002`.
- **Filename**: `NNNNNN-kebab-case-name.yml` in the matching folder, using the
  **same 6-digit number** as the identifier. Keep the filename number and the
  identifier number in sync. Example: `ecc/molec/000002-mycn-amplification.yml`.
- Choose the next unused number by listing the target folder (e.g. `ls ecc/molec`).

## 2. Choose the `state` and its required fields

`state` is the enum tag (`crates/ecc/src/lib.rs`, `#[serde(tag = "state")]`,
`deny_unknown_fields` — so **no extra/unknown keys** are allowed). The state
determines which fields are required:

| `state`       | Required fields                                                        |
| ------------- | --------------------------------------------------------------------- |
| `draft`       | none — every common field is optional (use while still filling it in) |
| `proposed`    | all common fields                                                     |
| `provisional` | all common fields                                                     |
| `adopted`     | all common fields **plus** `adoption_date` (RFC 3339 timestamp)       |

**Common fields** (from `crates/ecc/src/common.rs`), required for
proposed/provisional/adopted:

- `name` — string, human-readable title.
- `identifier` — the `ECC-…` identifier string (see step 1).
- `rfc` — a GitHub issue URL that MUST match
  `https://github.com/stjudecloud/ecc/issues/<number>` (enforced by regex in
  `crates/ecc/src/rfc.rs`).
- `description` — string; may be Markdown (block scalar `|` recommended).
- `values` — the value kind (see step 3).
- `references` — optional list; if present it must be **non-empty** (see step 4).

## 3. Define `values` (the value kind)

`values.kind` is the enum tag (`crates/ecc/src/common/value/kind.rs`,
`#[serde(tag = "kind")]`). One of:

- **`binary`** — a true/false determination (e.g. present/absent,
  amplified/not amplified). Requires a `description` with a `"true"` and a
  `"false"` entry, each an object with `summary` and `details`. **Quote the
  `"true"`/`"false"` keys** so YAML does not parse them as booleans. `summary`
  and `details` are sentences and must be non-empty.

  ```yaml
  values:
    kind: binary
    description:
      "true":
        summary: A one-line summary of the true case.
        details: |
          A longer explanation of what "true" means for this characteristic.
      "false":
        summary: A one-line summary of the false case.
        details: |
          A longer explanation of what "false" means for this characteristic.
  ```

- **`categorical`** — a fixed set of string options:

  ```yaml
  values:
    kind: categorical
    options:
      - Option A
      - Option B
  ```

- **`numerical`** — a measured number with a type and units. `type` is one of
  `signed`, `unsigned`, or `float`:

  ```yaml
  values:
    kind: numerical
    type: float
    units: percent
  ```

## 4. Optional `references`

If included, `references` is a non-empty list. Each entry has a `kind` of
`manuscript` or `preprint` (`crates/ecc/src/common/reference.rs`) with:
`title` (string), `authors` (string), `context` (a non-empty sentence),
`url` (any valid URL), and `highlighted` (bool). Omit the whole `references`
key if you have none — do not write an empty list.

```yaml
references:
  - kind: manuscript
    title: The title of the paper.
    authors: A. Author, B. Author
    context: One sentence on why this paper is relevant to this ECC.
    url: https://pubmed.ncbi.nlm.nih.gov/123456
    highlighted: true
```

## 5. Full template (proposed, binary)

```yaml
state: proposed
name: <Human Readable Name>
identifier: ECC-MOLEC-NNNNNN
rfc: https://github.com/stjudecloud/ecc/issues/<n>
description: |
  # Overview

  A Markdown description of the characteristic.
values:
  kind: binary
  description:
    "true":
      summary: <summary of the true case>.
      details: |
        <details of the true case>
    "false":
      summary: <summary of the false case>.
      details: |
        <details of the false case>
```

`ecc/000000-example.yml` is a complete, valid reference example.

## 6. Validate

Always validate after writing the file. The checker globs `ecc/**/*.yml` and
deserializes each into `Characteristic`, printing `OK` or `FAIL`. The Cargo
workspace lives in `crates/`, so run it from the repo root with an explicit
manifest path (this matches CI's `ecc-cli check ecc`, which installs the binary
first and runs from the root):

```bash
cargo run --manifest-path crates/Cargo.toml -p ecc-cli -- check ecc
```

If a file `FAIL`s, read the diagnostic (it points at the offending location),
compare against the schema above, and fix — common causes: unquoted
`true`/`false` keys, wrong identifier padding or type, an `rfc` URL that is not a
`stjudecloud/ecc` issue, unknown/extra keys (`deny_unknown_fields`), or an empty
`references` list.
