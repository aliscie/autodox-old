FROM fedora:latest

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

RUN dnf install -y \
        gcc \
        zstd \
        llvm \
        clang \
        openssl \
	openssl-devel \
	perl
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install tauri-cli
RUN rustup target add wasm32-unknown-unknown

RUN dnf module install nodejs:18/common -y
RUN dnf install yarnpkg -y

WORKDIR /usr/src/myapp

RUN cargo install wasm-pack

# tauri only depedency
RUN dnf install -y webkit2gtk4.0-devel.x86_64 \
    openssl-devel \
    curl \
    wget \
    libappindicator-gtk3 \
    librsvg2-devel \
    cmake
RUN dnf group install -y "C Development Tools and Libraries"
RUN dnf install patch -y

COPY . .
RUN cd ./frontend && yarn install
RUN cd ./frontend && yarn run wasm
RUN cd tauri-src && cargo build
CMD ["cargo", "tauri" , "dev"]
