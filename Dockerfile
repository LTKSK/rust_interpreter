# ===========================================
# build stage
FROM rust:1.46 AS builder

WORKDIR /usr/src/rust_interpreter
RUN rustup target add x86_64-unknown-linux-musl
COPY . .

RUN cargo install --target x86_64-unknown-linux-musl --path .

# ===========================================
# bundle stage
FROM scratch

COPY --from=builder /usr/local/cargo/bin/rust_interpreter .
CMD ["./rust_interpreter"]

