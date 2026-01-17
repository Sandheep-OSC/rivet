# Rivet

**A powerful workflow orchestration system built in Rust**

Rivet is a declarative workflow orchestration system that enables you to define, execute, and manage complex multi-step workflows with features like dependencies, retries, approvals, and rollbacks.

## ✨ Features

- **🔄 Step Dependencies** - Define complex workflow relationships with `depends_on`
- **🔁 Retry Logic** - Configurable retries with specific failure conditions
- **✅ Approval Workflows** - Required approvals with reviewer assignments
- **🔙 Rollback Mechanisms** - Automatic rollback procedures for failed steps
- **📋 Schema Validation** - Strict JSON Schema validation for workflow definitions
- **🏗️ Microservice Architecture** - Separate control plane and execution engine
- **🗄️ PostgreSQL Storage** - Reliable data persistence and state management

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+ (2024 edition)
- PostgreSQL 14+
- Docker & Docker Compose
- Just task runner
- pnpm (for frontend/TypeScript components)

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd rivet
   ```

2. **Install dependencies**
   ```bash
   # Install Rust dependencies
   cargo build
   
   # Install Node.js dependencies
   just js-install
   ```

3. **Set up the database**
   ```bash
   # Start PostgreSQL and pgAdmin
   docker-compose up -d
   
   # Copy environment file and configure
   cp .env.example .env
   # Edit .env with your database credentials
   ```

4. **Run your first workflow**
   ```bash
   # Start the control plane (management API)
   just rust-run control-plane
   
   # In another terminal, start the runner (execution engine)
   just rust-run runner
   ```

## 📖 Workflow Format

Rivet uses a declarative TOML format for workflow definitions:

```toml
name = "build-test-deploy"
version = 1

[steps.build]
run = ["npm install", "npm run build"]

[steps.test]
depends_on = ["build"]
run = ["npm test"]

[steps.deploy]
depends_on = ["test"]
run = ["./deploy.sh"]
retry = { max_attempts = 3, on = ["infra_error", "timeout"] }
approval = { required = true, reviewers = ["devops"] }
rollback = { run = ["./rollback.sh"] }
```

### Workflow Elements

- **`name`** - Human-readable workflow identifier
- **`version`** - Schema version (currently v1)
- **`steps`** - Collection of workflow steps
- **`run`** - Commands to execute for each step
- **`depends_on`** - List of step dependencies
- **`retry`** - Retry configuration with max attempts and conditions
- **`approval`** - Approval requirements with reviewer assignments
- **`rollback`** - Commands to execute on step failure

## 🏗️ Architecture

Rivet follows a microservice architecture with clear separation of concerns:

```
┌─────────────────┐    ┌─────────────────┐
│   Control Plane │    │      Runner     │
│                 │    │                 │
│ • Management API│    │ • Execution     │
│ • Monitoring    │◄──►│ • State Mgmt    │
│ • Workflow Ctrl │    │ • Retry Logic   │
└─────────────────┘    └─────────────────┘
         │                       │
         └───────────┬───────────┘
                     │
         ┌─────────────────┐
         │   PostgreSQL    │
         │                 │
         │ • Workflow State│
         │ • Execution Log │
         │ • Metadata      │
         └─────────────────┘
```

### Components

- **Control Plane** (`apps/control-plane/`) - Management interface and API server
- **Runner** (`apps/runner/`) - Workflow execution engine
- **Engine** (`packages/engine/`) - Core workflow processing and validation
- **Storage** (`packages/storage/`) - Data persistence layer
- **Contracts** (`packages/contracts/`) - Schema definitions and validation rules

## 🛠️ Development

### Commands

Use the Just task runner for common development tasks:

```bash
# Build all components
just build

# Run all tests
just test

# Format code (Rust + TypeScript)
just format

# Run linting
just lint

# Full CI pipeline
just ci

# Run specific applications
just rust-run control-plane
just rust-run runner

# Database operations
just rust-run-migration
```

### Project Structure

```
rivet/
├── apps/                    # Executable applications
│   ├── control-plane/       # Workflow management API
│   └── runner/             # Workflow execution engine
├── packages/               # Shared libraries
│   ├── engine/             # Core workflow processing
│   ├── storage/            # Data persistence layer
│   └── contracts/          # Schema definitions
├── docker-compose.yml      # Local development environment
├── Justfile               # Task runner configuration
└── Cargo.toml             # Rust workspace configuration
```

### How to Add a New Component

#### Adding a New Rust Service

1. Create a new directory under `apps/` (for executables) or `packages/` (for libraries).
2. Add a `Cargo.toml` file with appropriate dependencies.
3. Update the root `Cargo.toml` to include the new member in the `[workspace]` `members` array.
4. Implement your service logic.
5. Add tests and documentation as needed.

#### Adding a New JS/TS Package

1. Create a new directory under `packages/`.
2. Create a `package.json` with name, version, scripts, and dependencies.
3. Update `pnpm-workspace.yaml` to include the new package path.
4. Add a `tsconfig.json` extending the root `tsconfig.base.json`.
5. Implement your package logic.
6. Add tests and linting as needed.

### Code Quality

- **Rust**: Uses 2024 edition with strict Clippy rules (no unwraps, no panics)
- **TypeScript**: Formatted and linted with Biome
- **Testing**: Comprehensive test coverage with fixtures
- **Validation**: Schema-first development with JSON Schema

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run the full test suite (`just ci`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Development Guidelines

- Follow the existing code style and conventions
- Add tests for new functionality
- Update documentation as needed
- Ensure all CI checks pass before submitting

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Documentation](https://docs.rivet.dev) (coming soon)
- [Issues](https://github.com/your-org/rivet/issues)
- [Discussions](https://github.com/your-org/rivet/discussions)

---

**Built with ❤️ in Rust**