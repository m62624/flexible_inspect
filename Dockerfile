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
    rustup target add aarch64-apple-darwin; 

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
RUN pip3 install ziglang && pip install maturin && pip3 install twine

# Delete cache
RUN rm -rf /var/cache/apk/*
# Copy project
COPY . .
CMD ["echo","Welcome to the project build :D"]