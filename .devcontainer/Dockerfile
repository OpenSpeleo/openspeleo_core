FROM python:3.14.0-bookworm

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# Follow Bookworm package updates for this development environment.
# hadolint ignore=DL3008
RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    curl \
    wget \
    unzip \
    vim \
    nano \
    zsh \
    tmux \
    # cleaning up unused files
    && apt-get clean \
    && apt-get purge -y --auto-remove -o APT::AutoRemove::RecommendsImportant=false \
    && rm -rf /var/lib/apt/lists/*

# devcontainer dependencies and utils
# Follow Bookworm package updates for this development environment.
# hadolint ignore=DL3008
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    libclang-dev \
    # cleaning up unused files
    && apt-get clean \
    && apt-get purge -y --auto-remove -o APT::AutoRemove::RecommendsImportant=false \
    && rm -rf /var/lib/apt/lists/*

# Install Rust & Cargo
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain stable -y

# Install uv.
COPY --from=ghcr.io/astral-sh/uv:latest /uv /bin/uv

ENV UV_CACHE_DIR="/workspace/.uv/cache"
