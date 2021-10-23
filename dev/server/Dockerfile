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

RUN cargo build --release --locked --workspace

FROM archlinux

RUN yes | pacman -Syu
RUN yes | pacman -S postgresql-libs

WORKDIR /app

RUN mkdir dev
COPY dev/firecracker dev/firecracker
COPY dev/rootfs.img dev/rootfs.img
COPY dev/vmlinux.bin dev/vmlinux.bin

COPY --from=builder /app/target/release/doxa /app/doxa
COPY --from=builder /app/target/release/doxa_adm /app/doxa_adm

EXPOSE 3001

ENTRYPOINT ["/app/doxa"]
