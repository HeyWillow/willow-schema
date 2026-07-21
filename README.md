# willow-schema

`willow-schema` defines the existing Willow configuration v1 serialized
contracts. The crate provides the flat application configuration at
`willow_schema::config::v1::Config` and the NVS provisioning document at
`willow_schema::nvs::v1::Config`.

The runtime crate is unconditionally `no_std` and uses `alloc` for owned wire
values. It supports `no_std` environments with an allocator; it is not intended
for allocation-free environments.

## Scope and non-goals

This repository contains configuration data contracts only. It does not
contain runtime protocol or notification messages, WIS messages, REST or
database representations, ESP-IDF integration, resolved runtime configuration,
or consumer-specific recovery policies. The NVS provisioning document models
serialized values shared between WAS and Willow, not ESP-IDF storage mechanics.

Runtime messages belong in a separate `willow-protocol` project if they later
need to be shared.

## Source of truth

Rust types, Serde attributes, Rust doc comments, and explicit Rust provisioning
functions are authoritative. Wire compatibility is covered by Rust tests.
Externally sourced deployed documents may be retained as golden fixtures when
they provide an independent compatibility reference. See the respective `v1`
configuration types for exact wire semantics.

JSON Schema draft 2020-12 documents generated from the deserialization
contracts are committed under `generated/json-schema`. They are derived
artifacts for non-Rust consumers, not an independent source of truth. The NVS
SSID schema uses `x-willow-max-utf8-bytes` because standard JSON Schema string
length keywords count characters rather than encoded bytes. Its standard
`maxLength` remains useful as a necessary, but not sufficient, byte-length
constraint.

Integer schemas retain Schemars' non-standard `uint8`, `uint16`, and `uint32`
format annotations. Strict validators must register or ignore those formats.
The standard `minimum` and `maximum` keywords define the accepted ranges.

There are no generated language bindings in this revision.

This README is included as the crate-level Rust documentation; equivalent
prose is not maintained separately in `src/lib.rs`.

## Development

```sh
cargo fmt --check
cargo test --workspace --all-features
cargo xtask json-schema --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
rustup target add thumbv7em-none-eabi
cargo check --lib --target thumbv7em-none-eabi
```

Run the generator without `--check` after changing a serialized contract:

```sh
cargo xtask json-schema
```

## Compatibility and versioning

The `v1` module names the existing unversioned wire format; it does not add a
version property to serialized documents. Production Willow firmware and the
production Python WAS define deployed v1 behavior. WAS-RS is supporting
evidence where it agrees with those implementations.

Changes to field names, representations, enum values, or optionality require
compatibility tests. Externally sourced deployed documents may additionally be
retained as golden fixtures. Consumer migrations are deliberately outside this
repository's initial revision.
