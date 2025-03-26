FROM rust:1.84.1

WORKDIR /usr/local/bin
COPY wasm/gl.js wasm/index.html target/wasm32-unknown-unknown/release/sokoban.wasm ./

RUN cargo install basic-http-server

ENTRYPOINT ["/usr/local/cargo/bin/basic-http-server", "-a", "0.0.0.0:4000", "."]
