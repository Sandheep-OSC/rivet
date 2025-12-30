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
    if [ "{{dir}}" != "apps" ] && [ "{{dir}}" != "packages" ]; then
        echo "Error: Directory must be 'apps' or 'packages'"
        exit 1
    fi
    APP_DIR="{{dir}}/{{name}}"
    if [ -d "$$APP_DIR" ]; then
        echo "Error: App '{{name}}' already exists in {{dir}}!"
        exit 1
    fi
    cd {{dir}} && pnpm create vite {{name}}

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
    if [ "{{dir}}" != "apps" ] && [ "{{dir}}" != "packages" ]; then
        echo "Error: Directory must be 'apps' or 'packages'"
        exit 1
    fi
    APP_DIR="{{dir}}/{{name}}"
    if [ -d "$$APP_DIR" ]; then
        echo "Error: App '{{name}}' already exists in {{dir}}!"
        exit 1
    fi
    cargo new --lib {{dir}}/{{name}}
    # Add to workspace members in Cargo.toml
    WORKSPACE_PATH="{{dir}}/{{name}}"
    if ! grep -q "\"$$WORKSPACE_PATH\"" Cargo.toml; then
        # Add the new member before the closing bracket
        sed -i 's|\(members = \[.*\)\]|\1]|' Cargo.toml
        echo "Added $WORKSPACE_PATH to workspace members in Cargo.toml"
    else
        echo "$WORKSPACE_PATH already exists in workspace members"
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
