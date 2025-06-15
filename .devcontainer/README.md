# MRF-RS Dev Container

This directory contains the development container configuration for the MRF-RS project.

## Features

- **Rust Development Environment**: Latest stable Rust with cargo, rustfmt, clippy, and rust-analyzer
- **PostgreSQL Database**: PostgreSQL 16 with automatic setup and migrations
- **Development Tools**: 
  - sqlx-cli for database migrations
  - cargo-watch for auto-reloading
  - cargo-edit for dependency management
  - cargo-outdated for checking outdated dependencies
- **VS Code Extensions**: Pre-configured with Rust and database extensions

## Getting Started

1. **Prerequisites**:
   - Docker Desktop
   - Visual Studio Code
   - Dev Containers extension for VS Code

2. **Open in Dev Container**:
   - Open this project in VS Code
   - Press `F1` and select "Dev Containers: Reopen in Container"
   - Wait for the container to build (first time may take a few minutes)

3. **Environment Variables**:
   - The container automatically sets up:
     - `DATABASE_URL`: PostgreSQL connection string
     - `RUST_LOG`: Set to debug for development
     - `RUST_BACKTRACE`: Enabled for better error messages

## Database

- **Connection**: `postgresql://postgres:postgres@db:5432/mrf_db`
- **Migrations**: Automatically run on container creation
- **Access from host**: Port 5432 is forwarded to your local machine

## Common Commands

```bash
# Run the application
cargo run

# Run tests
cargo test

# Run with auto-reload
cargo watch -x run

# Run tests with auto-reload
cargo watch -x test

# Run database migrations
sqlx migrate run

# Create a new migration
sqlx migrate add <migration_name>

# Check code with clippy
cargo clippy

# Format code
cargo fmt

# Check for outdated dependencies
cargo outdated

# Add a new dependency
cargo add <package_name>
```

## Troubleshooting

### Database Connection Issues
If you encounter database connection issues:
```bash
# Check if PostgreSQL is running
pg_isready -h db -p 5432 -U postgres

# Manually run migrations
sqlx database create
sqlx migrate run
```

### Rebuilding the Container
If you need to rebuild the container:
1. Press `F1` in VS Code
2. Select "Dev Containers: Rebuild Container"

### Performance Issues
The container uses volume mounts for cargo cache and target directory to improve build performance. If you experience issues:
```bash
# Clear cargo cache
rm -rf /usr/local/cargo/registry
rm -rf /usr/local/cargo/git

# Clear target cache
cargo clean
```

## Customization

- **VS Code Settings**: Modify `.devcontainer/devcontainer.json` to add extensions or settings
- **Docker Configuration**: Update `docker-compose.yml` for additional services
- **Post-Create Script**: Edit `post-create.sh` to add custom setup steps 