FROM rust:1.46

WORKDIR /usr/src/rust_interpreter
COPY . .

RUN cargo build --release
CMD ["./target/release/rust_interpreter"]

