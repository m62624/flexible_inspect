#======================================================
# для проверки покрывающих тестов запускать контейнеры с этими параметрами 
# " --security-opt seccomp=unconfined "
# docker run --security-opt seccomp=unconfined --rm -it контейнер
#======================================================

# начальный образ
FROM rust:1.69

# рабочая директория проекта
WORKDIR /app

# скопировать содерижмое проекта в контекст (WORKDIR), где находится конфиг Dockerfile
SHELL ["/bin/bash", "-c"]

# устанавливаем нужные инструменты для сборки
RUN apt-get update && apt-get install -y build-essential zlib1g-dev \
    libncurses5-dev libgdbm-dev libnss3-dev libssl-dev \
    libreadline-dev libffi-dev libsqlite3-dev wget libbz2-dev

# Linter
RUN rustup component add clippy-preview 

# Крейт для проверки покрывающих тестов
RUN cargo install cargo-tarpaulin

# ставим targets для develop & build
RUN rustup target add x86_64-unknown-linux-gnu; \
    rustup target add aarch64-unknown-linux-gnu; \
    rustup target add x86_64-pc-windows-msvc; \ 
    rustup target add aarch64-pc-windows-msvc; \
    rustup target add x86_64-apple-darwin; \
    rustup target add aarch64-apple-darwin; 


# Версия пайтона указаны ниже cargo-tarpaulin (слои с tarpaulin долго обрабатывается) 
# Версия питона
ARG PYTHON_VERSION=3.10
# Полное название архива
ARG PYTHON_ARCHIVE=3.10.0
# Для запуска test & build 
ENV PYO3_PYTHON="/usr/local/bin/python${PYTHON_VERSION}"

# скачиваем архив с репозитория 
RUN wget https://www.python.org/ftp/python/$PYTHON_ARCHIVE/Python-$PYTHON_ARCHIVE.tgz \
    && tar -xzf Python-$PYTHON_ARCHIVE.tgz \
    && cd Python-$PYTHON_ARCHIVE \
    && ./configure --enable-optimizations \
    && make -j$(nproc) \
    && make altinstall

# ставим все, что связано с Python
RUN apt-get install python3-pip -y

# устанавливаем необходимые инструменты
RUN pip3 install maturin && pip3 install ziglang

# создаем слои для всех зависимостей 
COPY Cargo.toml Cargo.lock Makefile pyproject.toml ./
RUN mkdir src && touch src/lib.rs

# получаем кэш зависимостей 
RUN cargo fetch
# делаем один раз билд для каждой платформы, 
# headers загружаются только при сборке (либо я не нашел команду для maturin :D)
RUN make all;