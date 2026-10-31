#!/bin/sh
# Repeat the migration's automated checks with all three sibling checkouts.
set -eu
library=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
email=${DESERTUI_EMAIL_DIR:-"$library/../GPUI Email Client"}
vault=${DESERTUI_VAULT_DIR:-"$library/../GPUI Password Manager"}
for path in "$library" "$email" "$vault"; do
    if [ ! -f "$path/Cargo.toml" ]; then
        echo "Missing Cargo.toml: $path" >&2
        exit 1
    fi
done
(cd "$library" && cargo fmt --check && cargo test --locked && cargo clippy --locked --all-targets -- -D warnings)
(cd "$email" && cargo fmt --check && cargo test --locked && cargo test --locked --all-features && cargo clippy --locked --all-targets --all-features -- -D warnings && cargo build --locked --all-features)
(cd "$vault" && cargo fmt --check && cargo test --locked --workspace && cargo clippy --locked --workspace --all-targets --all-features -- -D warnings && cargo build --locked --workspace)
