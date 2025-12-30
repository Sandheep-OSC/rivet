set dotenv-load := true
set shell := ["bash", "-cu"]

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

rust-build:
    cargo build

rust-test:
    cargo test

rust-run app:
    cargo run -p {{app}}

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
