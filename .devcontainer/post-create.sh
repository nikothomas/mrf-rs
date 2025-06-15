#!/bin/bash
set -e

echo "🚀 Setting up MRF-RS development environment..."

# Wait for PostgreSQL to be ready
echo "⏳ Waiting for PostgreSQL to be ready..."
max_attempts=30
attempt=0
until pg_isready -h db -p 5432 -U postgres > /dev/null 2>&1; do
  attempt=$((attempt + 1))
  if [ $attempt -eq $max_attempts ]; then
    echo "❌ PostgreSQL failed to start after $max_attempts attempts"
    exit 1
  fi
  echo "PostgreSQL is unavailable - sleeping (attempt $attempt/$max_attempts)"
  sleep 2
done
echo "✅ PostgreSQL is ready!"

# Run database migrations
echo "🔄 Running database migrations..."
if [ -d "migrations" ]; then
    export DATABASE_URL="postgresql://postgres:postgres@db:5432/mrf_db"
    sqlx database create || true
    sqlx migrate run || echo "⚠️  Migration failed, but continuing setup"
    echo "✅ Database setup completed!"
else
    echo "⚠️  No migrations directory found, skipping migrations"
fi

# Build the project to ensure everything is set up
echo "🔨 Building project dependencies..."
cargo build --all-features || echo "⚠️  Build failed, but continuing setup"
echo "✅ Build step completed!"

# Run a simple cargo check instead of full tests
echo "🧪 Checking project..."
cargo check || echo "⚠️  Check failed, but setup is complete"

# Set up git hooks if needed
if [ -f ".githooks/pre-commit" ]; then
    echo "🔗 Setting up git hooks..."
    git config core.hooksPath .githooks
    echo "✅ Git hooks configured!"
fi

echo "🎉 Development environment setup complete!"
echo ""
echo "📝 Quick commands:"
echo "  cargo run              - Run the application"
echo "  cargo test             - Run tests"
echo "  cargo watch -x run     - Run with auto-reload"
echo "  cargo watch -x test    - Run tests with auto-reload"
echo "  sqlx migrate run       - Run database migrations"
echo "  cargo clippy           - Run linter"
echo "  cargo fmt              - Format code"
echo "" 