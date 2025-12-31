set dotenv-load := true
set shell := ["bash", "-cu"]

# ---------------------
# Global variables
# ---------------------

# Rust crate that generates TS types
RUST_BINDINGS_CRATE := "packages/contracts-rust"

# Directory to store generated TS types
GENERATED_TS_DIR := "packages/dummy-lib/src"




default:
    just --list

# ---------------------
# JS
# ---------------------

js-install:
    pnpm install

# Install packages in a specific app
js-install-app app:
    pnpm --filter {{app}} install

js-build:
    pnpm run build --workspaces

js-test:
    pnpm test --workspaces


# Run a specific JS project
js-run app:
    pnpm --filter {{app}} run dev

# Run multiple specific apps (space-separated)
js-run-apps apps:
    #!/usr/bin/env bash
    for app in {{apps}}; do
        pnpm --filter "$$app" run dev &
    done
    wait

# Create a new JS app
js-create-app name dir="apps":
    #!/usr/bin/env bash
    set -euo pipefail

    DIR="{{dir}}"

    # Strip optional "dir=" prefix
    if [[ "$DIR" == dir=* ]]; then
        DIR="${DIR#dir=}"
    fi

    case "$DIR" in
        apps|apps/*|packages|packages/*)
            ;;
        *)
            echo "Error: Directory must start with 'apps' or 'packages'"
            echo "Received: '$DIR'"
            exit 1
            ;;
    esac

    APP_DIR="$DIR/{{name}}"

    if [ -d "$APP_DIR" ]; then
        echo "Error: App '{{name}}' already exists in $DIR!"
        exit 1
    fi

    mkdir -p "$DIR"
    cd "$DIR"
    pnpm create vite {{name}}

# ---------------------
# Rust
# ---------------------

rust-run-migration:
  cargo run -p storage-migrations

rust-build:
    cargo build

rust-test:
    cargo test

rust-run app:
    cargo run -p {{app}}

# Add a dependency to a specific Rust crate
rust-add-dependency crate package *flags:
    cargo add --package {{crate}} {{package}} {{flags}}

# Add a dev dependency to a specific Rust crate
rust-add-dev-dependency crate package *flags:
    cargo add --package {{crate}} --dev {{package}} {{flags}}

# Create a new Rust app
rust-create-app name dir="packages":
    #!/usr/bin/env bash
    set -euo pipefail

    DIR="{{dir}}"

    # Strip optional "dir=" prefix
    if [[ "$DIR" == dir=* ]]; then
        DIR="${DIR#dir=}"
    fi

    case "$DIR" in
        apps|apps/*|packages|packages/*)
            ;;
        *)
            echo "Error: Directory must start with 'apps' or 'packages'"
            echo "Received: '$DIR'"
            exit 1
            ;;
    esac

    APP_DIR="$DIR/{{name}}"
    echo "Creating Rust crate at: $APP_DIR"

    if [ -d "$APP_DIR" ]; then
        echo "Error: App '{{name}}' already exists at $APP_DIR"
        exit 1
    fi

    mkdir -p "$DIR"
    cargo new "$APP_DIR"

    if ! grep -q "\"$APP_DIR\"" Cargo.toml; then
        sed -i.bak "/members = \[/a\\
        \"$APP_DIR\"," Cargo.toml
        echo "Added $APP_DIR to workspace members in Cargo.toml"
    else
        echo "$APP_DIR already exists in workspace members"
    fi

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


# Generate TS types from Rust crates using ts-rs
# ---------------------
# TS-RS / TypeScript bindings
# ---------------------

rust-generate-types:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "Generating TS types from {{RUST_BINDINGS_CRATE}}..."
    cargo test -p contracts-rust export_bindings

    echo "Clearing old TS types in {{GENERATED_TS_DIR}}..."
    rm -rf "{{GENERATED_TS_DIR}}"/*
    mkdir -p "{{GENERATED_TS_DIR}}"

    echo "Copying newly generated TS types..."
    cp -r "{{RUST_BINDINGS_CRATE}}/bindings/"* "{{GENERATED_TS_DIR}}/"

    echo "TS types generated and copied successfully."

# Build JS projects after generating types
js-build-with-types: rust-generate-types
    just js-build

# Test JS projects after generating types
js-test-with-types: rust-generate-types
    just js-test

# Full workflow: generate types, build Rust + JS
build-all: rust-generate-types
    just build

# Full workflow: generate types, test Rust + JS
test-all: rust-generate-types
    just test

# Optional: watch Rust crate for changes and regenerate types automatically
rust-watch-types:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "Watching {{RUST_BINDINGS_CRATE}}/src for changes..."
    cargo watch -w "{{RUST_BINDINGS_CRATE}}/src" -x "test -p contracts-rust export_bindings" \
        -s "rm -rf {{GENERATED_TS_DIR}}/* && cp -r {{RUST_BINDINGS_CRATE}}/bindings/* {{GENERATED_TS_DIR}}/ && echo 'TS types updated!'"
