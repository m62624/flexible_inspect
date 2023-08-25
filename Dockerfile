# Base image
FROM rust:latest
LABEL org.opencontainers.image.source="https://github.com/m62624/flexible_inspect"
# System libs
RUN apt-get update && apt-get install -y libffi-dev \ 
                       openssl \
                       libssl-dev \
                       pkg-config \
                       zlib1g-dev \
                       make \
                       build-essential \
                       g++;

# Rust wasm
RUN apt-get install -y nodejs npm && cargo install wasm-pack
# Rust tools
RUN cargo install cargo-tarpaulin && rustup component add clippy-preview  

# Python
RUN apt-get install -y python3-dev python3-pip python3-venv;
ENV VIRTUAL_ENV=/opt/venv
RUN python3 -m venv $VIRTUAL_ENV
ENV PATH="$VIRTUAL_ENV/bin:$PATH"
RUN pip3 install --upgrade pip; \
    pip3 install maturin[zig] && pip3 install twine

# Targets for rust
RUN rustup target add x86_64-unknown-linux-gnu; \
    rustup target add aarch64-unknown-linux-gnu; \
    rustup target add x86_64-pc-windows-msvc; \ 
    rustup target add aarch64-pc-windows-msvc; \
    rustup target add x86_64-apple-darwin; \
    rustup target add aarch64-apple-darwin; \
    rustup target add wasm32-unknown-unknown

WORKDIR /main_project
COPY Cargo.lock Cargo.toml Makefile .

ARG PROJECT_FOLDER_PY=flexible_inspect_py
ARG PROJECT_FOLDER_JS=flexible_inspect_js
ARG PROJECT_FOLDER_RS=flexible_inspect_rs

RUN mkdir -p $PROJECT_FOLDER_RS/src; touch $PROJECT_FOLDER_RS/src/lib.rs; \
    mkdir -p $PROJECT_FOLDER_PY/src; touch $PROJECT_FOLDER_PY/src/lib.rs; \
    mkdir -p $PROJECT_FOLDER_JS/src; touch $PROJECT_FOLDER_JS/src/lib.rs;
COPY $PROJECT_FOLDER_RS/Cargo.toml $PROJECT_FOLDER_RS/Cargo.toml
COPY $PROJECT_FOLDER_PY/Cargo.toml $PROJECT_FOLDER_PY/Cargo.toml
COPY $PROJECT_FOLDER_PY/pyproject.toml $PROJECT_FOLDER_PY/pyproject.toml
COPY $PROJECT_FOLDER_JS/Cargo.toml $PROJECT_FOLDER_JS/Cargo.toml

# show files
RUN ls -R
# Get dependency cache for building other targets (sdk.headers (windows))
RUN make all-python
# delete dummy projects
RUN rm -rf *

# Copy project
COPY . .