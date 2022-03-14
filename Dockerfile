FROM rust:slim-buster

WORKDIR /usr/src/load-striker
COPY . .

RUN apt update && apt install --yes pkg-config libssl-dev
RUN cargo build --release


FROM debian:buster-slim

VOLUME /data

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    libc6-dev \
    pkg-config \
    libssl-dev

COPY --from=0 /usr/src/load-striker/target/release/load-striker /

CMD ./load-striker -u "$CONCURRENT_USERS" -f "$TARGETS_FILE"