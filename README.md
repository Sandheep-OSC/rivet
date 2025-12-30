# Rivet

A monorepo combining Rust and JavaScript/TypeScript projects, featuring a Rust core library and React applications.

## Project Structure

```
rivet/
├── packages/
│   └── rust-core/          # Rust core library
├── apps/
│   └── react-dummy/        # React application (Vite + React 19)
├── Cargo.toml              # Rust workspace configuration
├── package.json            # Root package.json
├── pnpm-workspace.yaml     # pnpm workspace configuration
└── Justfile                # Build automation commands
```

## Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Node.js**: Version 18+ recommended
- **pnpm**: Install with `npm install -g pnpm`
- **just**: Install from [just.systems](https://just.systems/) (optional, for using Justfile commands)

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rivet
   ```

2. Install JavaScript dependencies:
   ```bash
   pnpm install
   ```

3. Rust dependencies are managed by Cargo and will be installed automatically on first build.

## Development

### Using Just (Recommended)

The project includes a `Justfile` with convenient commands:

- **List all available commands:**
  ```bash
  just
  ```

- **Build everything:**
  ```bash
  just build
  ```

- **Run tests:**
  ```bash
  just test
  ```

- **Run a specific React app:**
  ```bash
  just js-run react-dummy
  ```

- **Run multiple React apps in parallel:**
  ```bash
  just js-run-apps "react-dummy another-app"
  ```

- **Run a Rust package:**
  ```bash
  just rust-run rust-core
  ```

### App Management

- **Install packages for a specific JS app:**
  ```bash
  just js-install-app react-dummy
  ```

- **Create a new JS app:**
  ```bash
  # Create in apps/ directory (default)
  just js-create-app my-new-app
  
  # Create in packages/ directory
  just js-create-app my-new-app dir=packages
  
  # Create in nested directory
  just js-create-app my-new-app dir=apps/engine/frontend
  ```

- **Create a new Rust crate:**
  ```bash
  # Create in packages/ directory (default)
  just rust-create-app my-rust-crate
  
  # Create in apps/ directory
  just rust-create-app my-rust-crate dir=apps
  
  # Create in nested directory (automatically adds to workspace)
  just rust-create-app my-rust-crate dir=apps/engine/crates/storage
  ```

### Manual Commands

#### JavaScript/TypeScript

- **Install dependencies:**
  ```bash
  pnpm install
  ```

- **Build all workspaces:**
  ```bash
  pnpm run build --workspaces
  ```

- **Run a specific app:**
  ```bash
  pnpm --filter react-dummy run dev
  ```

- **Run tests:**
  ```bash
  pnpm test --workspaces
  ```

#### Rust

- **Build:**
  ```bash
  cargo build
  ```

- **Run:**
  ```bash
  cargo run -p rust-core
  ```

- **Test:**
  ```bash
  cargo test
  ```

## Clean

Remove all build artifacts and dependencies:

```bash
just clean
```

Or manually:
```bash
rm -rf node_modules
cargo clean
```

## Technologies

- **Rust**: Core library (Edition 2024)
- **React**: 19.2.0 with TypeScript
- **Vite**: Build tool (using rolldown-vite)
- **pnpm**: Package manager for JavaScript/TypeScript
- **Cargo**: Package manager for Rust
- **just**: Command runner for automation

## License

[Add your license here]

