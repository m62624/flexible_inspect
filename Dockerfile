# Base image
FROM rust:1.71-alpine
# Metadata
LABEL maintainer="m62624"
LABEL version_image="1.0.0"
LABEL description="Docker image for rust projects (includes build support for js (wasm), python (pyo3))"
LABEL repository="https://github.com/m62624/flexible_inspect"
 
WORKDIR main_project

# Targets for rust
RUN rustup target add x86_64-unknown-linux-gnu; \
    rustup target add aarch64-unknown-linux-gnu; \
    rustup target add x86_64-pc-windows-msvc; \ 
    rustup target add aarch64-pc-windows-msvc; \
    rustup target add x86_64-apple-darwin; \
    rustup target add aarch64-apple-darwin; \
    rustup target add wasm32-unknown-unknown

# System libs
RUN apk add --no-cache libffi-dev \ 
                       openssl-dev \
                       pkgconfig \
                       zlib-dev \
                       make \
                       gcc \
                       g++;

# Rust wasm
RUN apk add --no-cache nodejs && cargo install wasm-pack
# Rust tools
RUN cargo install cargo-tarpaulin && rustup component add clippy-preview  

# Python
RUN apk add --no-cache python3-dev py3-pip && pip3 install --upgrade pip
RUN pip install maturin[zig] && pip3 install twine

ARG PROJECT_FOLDER_PY=flexible_inspect_py
ARG PROJECT_FOLDER_JS=flexible_inspect_js
ARG PROJECT_FOLDER_RS=flexible_inspect_rs

# Delete cache
RUN rm -rf /var/cache/apk/*

COPY Cargo.toml .
COPY Cargo.lock .
COPY Makefile .

# Create dummy projects (Rust version)
RUN mkdir -p $PROJECT_FOLDER_RS/src; touch $PROJECT_FOLDER_RS/src/lib.rs
COPY $PROJECT_FOLDER_RS/Cargo.toml $PROJECT_FOLDER_RS/Cargo.toml

# Create dummy projects (Python version)
RUN mkdir -p $PROJECT_FOLDER_PY/src; touch $PROJECT_FOLDER_PY/src/lib.rs
COPY $PROJECT_FOLDER_PY/Cargo.toml $PROJECT_FOLDER_PY/Cargo.toml
COPY $PROJECT_FOLDER_PY/pyproject.toml $PROJECT_FOLDER_PY/pyproject.toml

# Create dummy projects (JavaScrpit version)
RUN mkdir -p $PROJECT_FOLDER_JS/src; touch $PROJECT_FOLDER_JS/src/lib.rs
COPY $PROJECT_FOLDER_JS/Cargo.toml $PROJECT_FOLDER_JS/Cargo.toml

# show files
RUN ls -R
# Get dependency cache for building other targets (sdk.headers (windows))
RUN make all-python
# delete dummy projects
RUN rm -rf *
# Copy project
COPY . .
CMD ["echo","Welcome to the project build :D"]