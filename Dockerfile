FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src
COPY target ./target

RUN cargo build --release


FROM debian:stable-slim

RUN useradd -m myuser
WORKDIR /app

COPY --from=builder /app/target/release/pacman /app/pacman
COPY --from=builder /app/target/release/Levels /app/Levels

USER myuser

