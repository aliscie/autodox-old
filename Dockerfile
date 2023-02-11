FROM fedora:latest

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add wasm32-unknown-unknown

RUN dnf install -y \
        gcc \
        zstd \
        llvm \
        clang \
        openssl \
        openssl-devel \
        perl \
        webkit2gtk4.0-devel \
        openssl-devel \
        curl \
        wget \
        libappindicator-gtk3 \
        librsvg2-devel \
        cmake \
        dbus-x11 \
        PackageKit-gtk3-module \
        libcanberra-gtk2 \
        patch && dnf module install -y nodejs:18/common  && dnf install -y yarnpkg && dnf group install -y "C Development Tools and Libraries"

RUN cargo install tauri-cli wasm-pack
WORKDIR /usr/src/myapp
COPY . .
# installing frontend files
RUN cd ./frontend && yarn install && wasm-pack build . --target=web --dev
RUN cargo tauri build --debug && mv /usr/src/myapp/target/debug/tauri /usr/bin/autodox && rm -rf /usr/src/myapp
RUN  dbus-uuidgen> /etc/machine-id
CMD ["/usr/bin/autodox"]
