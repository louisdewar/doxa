#FROM rust:latest as chef
#
#RUN cargo install cargo-chef
#WORKDIR app
#
#FROM chef as planner
#
#COPY . .
#RUN cargo chef prepare  --recipe-path recipe.json
#
#FROM chef AS builder
#COPY --from=planner /app/recipe.json recipe.json
#
#RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:latest as builder
WORKDIR /app

COPY ./src src
COPY ./crates crates
COPY ./competitions competitions
COPY ./migrations migrations
COPY ./Cargo.toml .
COPY ./Cargo.lock .

RUN cargo build --release --locked

FROM scratch
COPY --from=builder /app/target/release/doxa .
