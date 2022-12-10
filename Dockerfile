FROM rust:1.31

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

RUN apk add --no-cache \
        ca-certificates \
        gcc \
        zstd \
        llvm \
        clang \
        openssl \
        curl -sSf https://install.surrealdb.com | sh # install surrealdb


CMD ["cargo tauri dev"]
