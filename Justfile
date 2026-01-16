set dotenv-load := true
set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

# ==================================================
# Configuration
# ==================================================

RUST_BINDINGS_CRATE := "packages/contracts-rust"
GENERATED_TS_DIR    := "packages/dummy-lib/src"

# ==================================================
# Default / Help
# ==================================================

default:
    just --list

# ==================================================
# Setup / Install
# ==================================================

install:
    just js-install

js-install:
    pnpm install

js-install-app app:
    pnpm --filter {{app}} install

# ==================================================
# Formatting
# ==================================================

format:
    just format-rust
    just format-js

format-rust:
    cargo fmt --all

format-js:
    pnpm biome format . --write

# ==================================================
# Linting
# ==================================================

lint:
    just lint-rust
    just lint-js

lint-rust:
    cargo clippy --all-targets --all-features -- -D warnings

lint-js:
    pnpm biome lint .

# ==================================================
# Static Checks (no mutation)
# ==================================================

check:
    just check-rust
    just check-js

check-rust:
    cargo check --all-targets --all-features

check-js:
    pnpm biome check .

# ==================================================
# CI
# ==================================================

ci:
    just format
    just lint
    just check

# ==================================================
# Build / Test (Combined)
# ==================================================

build:
    just build-js
    just build-rust

test:
    just test-js
    just test-rust

clean:
    rm -rf node_modules
    cargo clean

# ==================================================
# JavaScript / TypeScript
# ==================================================

build-js:
    pnpm run build --workspaces

test-js:
    pnpm test --workspaces

run-js app:
    pnpm --filter {{app}} run dev

run-js-multi apps:
    #!/usr/bin/env bash
    for app in {{apps}}; do
        pnpm --filter "$$app" run dev &
    done
    wait

create-js-app name dir="apps":
    #!/usr/bin/env bash
    set -euo pipefail

    DIR="{{dir}}"
    [[ "$DIR" == dir=* ]] && DIR="${DIR#dir=}"

    case "$DIR" in
        apps|apps/*|packages|packages/*) ;;
        *)
            echo "Error: Directory must start with 'apps' or 'packages'"
            exit 1
            ;;
    esac

    APP_DIR="$DIR/{{name}}"
    [ -d "$APP_DIR" ] && echo "App already exists: $APP_DIR" && exit 1

    mkdir -p "$DIR"
    cd "$DIR"
    pnpm create vite {{name}}

# ==================================================
# Rust
# ==================================================

build-rust:
    cargo build

test-rust *ARGS:
    cargo test {{ARGS}}

run-rust app:
    cargo run -p {{app}}

run-rust-migration:
    cargo run -p storage-migrations

add-rust-dep crate package *flags:
    cargo add --package {{crate}} {{package}} {{flags}}

add-rust-dev-dep crate package *flags:
    cargo add --package {{crate}} --dev {{package}} {{flags}}

create-rust-app name dir="packages":
    #!/usr/bin/env bash
    set -euo pipefail

    DIR="{{dir}}"
    [[ "$DIR" == dir=* ]] && DIR="${DIR#dir=}"

    case "$DIR" in
        apps|apps/*|packages|packages/*) ;;
        *)
            echo "Error: Directory must start with 'apps' or 'packages'"
            exit 1
            ;;
    esac

    APP_DIR="$DIR/{{name}}"
    [ -d "$APP_DIR" ] && echo "Crate already exists: $APP_DIR" && exit 1

    cargo new "$APP_DIR"

    if ! grep -q "\"$APP_DIR\"" Cargo.toml; then
        sed -i.bak "/members = \[/a\\
        \"$APP_DIR\"," Cargo.toml
        echo "Added $APP_DIR to workspace"
    fi

# ==================================================
# Rust → TypeScript Bindings (ts-rs)
# ==================================================

generate-types:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "Generating TS bindings from {{RUST_BINDINGS_CRATE}}..."
    cargo test -p contracts-rust export_bindings

    echo "Refreshing {{GENERATED_TS_DIR}}..."
    rm -rf "{{GENERATED_TS_DIR}}"/*
    mkdir -p "{{GENERATED_TS_DIR}}"

    cp -r "{{RUST_BINDINGS_CRATE}}/bindings/"* "{{GENERATED_TS_DIR}}/"
    echo "TypeScript bindings updated."

watch-types:
    #!/usr/bin/env bash
    set -euo pipefail

    cargo watch \
        -w "{{RUST_BINDINGS_CRATE}}/src" \
        -x "test -p contracts-rust export_bindings" \
        -s "rm -rf {{GENERATED_TS_DIR}}/* && cp -r {{RUST_BINDINGS_CRATE}}/bindings/* {{GENERATED_TS_DIR}}/"

# ==================================================
# Workflows
# ==================================================

build-with-types: generate-types build
test-with-types:  generate-types test
build-all:        generate-types build
test-all:         generate-types test
