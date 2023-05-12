# начальный образ
FROM rust
# рабочая директория проекта
WORKDIR /app
# скопировать содерижмое проекта в контекст (WORKDIR), где находится конфиг dockerfile
SHELL ["/bin/bash", "-c"]
# обновляем зеркала
RUN apt-get update;
# ставим targets для develop & build
RUN rustup target add x86_64-unknown-linux-gnu; \
    rustup target add x86_64-pc-windows-msvc; \ 
    rustup target add aarch64-pc-windows-msvc; \
    rustup target add x86_64-apple-darwin; \
    rustup target add aarch64-apple-darwin;
# ставим всё, что связано с python3
RUN apt-get install build-essential zlib1g-dev \
    libncurses5-dev libgdbm-dev libnss3-dev libssl-dev \
    libreadline-dev libffi-dev libsqlite3-dev wget libbz2-dev -y;
# ставим сам python 3 и его окружение
RUN apt-get install python3.10 python3-pip -y;
# врубаем окружение & ставим для сборки в wheels
RUN pip install maturin && pip install ziglang;
# делаем слои для всех зависимостей 
COPY Cargo.toml Cargo.lock Makefile pyproject.toml .
RUN mkdir src; touch src/lib.rs;
RUN cargo fetch;
RUN make all;
#====================================================
RUN rm -rf *