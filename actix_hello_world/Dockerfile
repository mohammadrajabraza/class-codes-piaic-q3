FROM rust:1-slim-buster AS base

WORKDIR /app
RUN cargo init
COPY Cargo.toml /app/Cargo.toml
RUN cargo fetch

COPY . /app/

CMD [ "cargo", "test", "--offline" ]

FROM base AS builder

RUN cargo build --release --offline

FROM rust:1-slim-buster

COPY --from=builder /app/target/release/actix_hello_world /usr/bin/actix_hello_world

EXPOSE 3000

ENTRYPOINT [ "/usr/bin/actix_hello_world" ]
