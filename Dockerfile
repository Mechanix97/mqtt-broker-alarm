FROM rust:1.87 AS chef

RUN apt-get update && apt-get install -y \
    build-essential \
    libclang-dev \
    libc6 \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef

WORKDIR /mqtt-broker-alarm

FROM chef AS planner
COPY . .
# Determine the crates that need to be built from dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /mqtt-broker-alarm/recipe.json recipe.json
# Build dependencies only, these remain cached
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder mqtt-broker-alarm/target/release/mqtt-broker-alarm .
ENTRYPOINT [ "./mqtt-broker-alarm" ]
