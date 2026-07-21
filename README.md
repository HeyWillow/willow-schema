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

There are no generated JSON Schema or TypeScript artifacts in this revision.

This README is included as the crate-level Rust documentation; equivalent
prose is not maintained separately in `src/lib.rs`.

## Development

```sh
cargo fmt --check
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
rustup target add thumbv7em-none-eabi
cargo check --lib --target thumbv7em-none-eabi
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
