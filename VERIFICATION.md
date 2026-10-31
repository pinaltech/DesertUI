# Verification

October 30, 2026, Apple Silicon macOS, Rust 1.96.0.

Initial shared crate: `cargo test --locked`, `cargo fmt --check`, and
`cargo clippy --locked --all-targets -- -D warnings` pass. Tests validate all
layout-policy boundaries, every accent's contrast and preference keys, and real
GPUI click/layout behavior at eight viewport sizes from 1440×1000 to 360×320.
The gallery is compiled by the all-target Clippy check.

Before migration, Ironwood's workspace suite passes. Picacho's default and
serial all-feature baseline runs both fail eight existing local TLS reliability
tests (49 pass, 2 opt-in tests ignored); all its UI tests pass. Final migration
checks and commit references will be recorded after application validation.
