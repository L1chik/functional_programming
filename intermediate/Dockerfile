FROM rust:1.68.2 as builder

RUN cargo new --bin intermediate
WORKDIR ./intermediate
COPY ./Cargo.toml  ./Cargo.toml
COPY ./src ./src
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /intermediate/target/release/intermediate ./intermediate
CMD ["./intermediate"]