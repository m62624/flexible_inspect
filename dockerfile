# Установка начального образа (фиксируем, чтобы сюрпризов не было)
FROM rust:1.69

# Данные пользователя
ARG USERNAME=user
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Установка sudo
RUN apt-get update && apt-get install -y sudo

# Создание пользователя и добавление в группу sudo
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && echo "$USERNAME ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME
# Настройка рабочего пользователя
USER $USERNAME
# Установка рабочей директории проекта
WORKDIR /home/$USERNAME/app

# Установка зависимостей Python 3.10
RUN sudo apt-get install -y build-essential zlib1g-dev libncurses5-dev libgdbm-dev libnss3-dev \
    libssl-dev libreadline-dev libffi-dev libsqlite3-dev wget libbz2-dev

# Скачивание и установка Python 3.10
RUN wget https://www.python.org/ftp/python/3.10.0/Python-3.10.0.tgz \
    && sudo tar -xzf Python-3.10.0.tgz \
    && cd Python-3.10.0 \
    && sudo ./configure --enable-optimizations \
    && sudo make -j$(nproc) \
    && sudo make altinstall

# Установка pip для Python 3.10
RUN python3.10 -m ensurepip --upgrade \
    && python3.10 -m pip install --upgrade pip

# Установка необходимых инструментов
RUN pip3.10 install maturin && pip3.10 install ziglang

# Установка переменных окружения
ENV PATH="/home/$USERNAME/.cargo/bin:${PATH}"
ENV PYTHON=/usr/bin/python3.10
ENV PYO3_PYTHON=/usr/bin/python3.10

# установка инструментов для каждой архитектуры
RUN rustup target add x86_64-unknown-linux-gnu; \
    rustup target add aarch64-unknown-linux-gnu; \
    rustup target add x86_64-pc-windows-msvc; \ 
    rustup target add aarch64-pc-windows-msvc; \
    rustup target add x86_64-apple-darwin; \
    rustup target add aarch64-apple-darwin;

# Установка инструмента для покрытия тестами
RUN cargo install cargo-tarpaulin

# Копирование файлов зависимостей и сборка
COPY Cargo.toml Cargo.lock Makefile pyproject.toml .
RUN mkdir src && touch src/lib.rs
RUN cargo fetch
RUN make all
RUN rm -rf *
COPY . .
