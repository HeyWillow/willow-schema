# willow-schema

`willow-schema` defines the existing Willow configuration v1 serialized
contract. The crate provides the flat wire-compatible Rust type at
`willow_schema::config::v1::Config` and the enums used by that document.

The runtime crate is unconditionally `no_std` and uses `alloc` for owned wire
values. It supports `no_std` environments with an allocator; it is not intended
for allocation-free environments.

## Scope and non-goals

This initial revision contains configuration data contracts only. It does not
contain runtime protocol or notification messages, WIS messages, REST or
database representations, ESP-IDF integration, NVS documents, resolved runtime
configuration, or consumer-specific recovery policies.

Runtime messages belong in a separate `willow-protocol` project if they later
need to be shared.

## Source of truth

Rust types, Serde attributes, Rust doc comments, and
`Config::was_provisioning_defaults()` are authoritative. Wire compatibility is
covered by Rust tests. Externally sourced deployed documents may be retained as
golden fixtures when they provide an independent compatibility reference. See
`willow_schema::config::v1::Config` for exact wire semantics.

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
