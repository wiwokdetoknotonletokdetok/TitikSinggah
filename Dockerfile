FROM rustlang/rust:nightly-slim AS builder

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/titiksinggah /usr/local/bin/titiksinggah

EXPOSE 5000

ENV RUST_LOG=info

CMD ["titiksinggah"]
