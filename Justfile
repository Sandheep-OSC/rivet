set dotenv-load := true
set shell := ["bash", "-cu"]

default:
    just --list

# ---------------------
# JS
# ---------------------

js-install:
    pnpm install

js-build:
    pnpm run build --workspaces

js-test:
    pnpm test --workspaces


# Run a specific JS project
js-run app:
    pnpm --filter {{app}} run dev

# ---------------------
# Rust
# ---------------------

rust-build:
    cargo build

rust-test:
    cargo test

rust-run app:
    cargo run -p {{app}}

# ---------------------
# Combined
# ---------------------

build:
    just js-build
    just rust-build

test:
    just js-test
    just rust-test

clean:
    rm -rf node_modules
    cargo clean
