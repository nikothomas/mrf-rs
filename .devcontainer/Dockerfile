FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

# Install additional packages
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        postgresql-client \
        libssl-dev \
        pkg-config \
        build-essential \
        curl \
        git \
        vim \
        htop \
        ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Switch to root for tool installation
USER root

# Install sqlx-cli for database migrations
RUN cargo install sqlx-cli --no-default-features --features rustls,postgres

# Install additional Rust tools
RUN rustup component add rustfmt clippy rust-analysis rust-src \
    && cargo install cargo-watch cargo-edit cargo-outdated

# Create non-root user
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Create the user if it doesn't exist
RUN if ! id -u $USERNAME > /dev/null 2>&1; then \
        groupadd --gid $USER_GID $USERNAME \
        && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
        && apt-get update \
        && apt-get install -y sudo \
        && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
        && chmod 0440 /etc/sudoers.d/$USERNAME; \
    fi

# Set the default user
USER $USERNAME

# Set working directory
WORKDIR /workspace 