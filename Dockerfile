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

COPY ./shared shared
COPY ./editor ./editor
COPY ./tauri-src ./tauri-src
COPY ./canisters ./canisters
COPY ./webscoket_server ./webscoket_server
# copying frontend files
COPY ./frontend/package.json ./frontend/package.json
COPY ./frontend/yarn.lock ./frontend/yarn.lock
COPY ./frontend/Cargo.toml ./frontend/Cargo.toml
COPY ./frontend/Cargo.toml ./frontend/Cargo.toml
COPY ./frontend/public ./frontend/public
COPY ./frontend/backend ./frontend/backend
COPY ./frontend/index.js ./frontend/index.js
COPY ./frontend/index.html ./frontend/index.html
COPY ./frontend/src ./frontend/src
COPY ./frontend/icons ./frontend/icons
COPY ./frontend/vite.config.ts ./frontend/vite.config.ts

# installing frontend files
RUN cd ./frontend && yarn install
RUN cd ./frontend && wasm-pack build . --target=web --dev

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo tauri build --debug
RUN mv ./target/debug/tauri /usr/bin/autodox
RUN rm -rf /usr/src/myapp
CMD ["/usr/bin/autodox"]
