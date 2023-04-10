FROM rust:1.68-buster as builder
RUN mkdir /code
WORKDIR /code

RUN apt update && apt install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

RUN mkdir -p client/tuat-feed/src common/common/src server/feed-scraper/src server/server/src
COPY Cargo.lock Cargo.toml ./
COPY client/tuat-feed/Cargo.toml client/tuat-feed/Cargo.toml
COPY common/common/Cargo.toml common/common/Cargo.toml
COPY server/feed-scraper/Cargo.toml server/feed-scraper/Cargo.toml
COPY server/server/Cargo.toml server/server/Cargo.toml
RUN touch client/tuat-feed/src/lib.rs \
    common/common/src/lib.rs \
    server/feed-scraper/src/lib.rs \
    server/server/src/lib.rs
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN cargo build --release --target x86_64-unknown-linux-musl

COPY . ./
# update access time
RUN touch client/tuat-feed/src/lib.rs \
    common/common/src/lib.rs \
    server/feed-scraper/src/lib.rs \
    server/server/src/lib.rs
RUN cargo install --target x86_64-unknown-linux-musl --path ./server/server/ --root output 
RUN strip /code/output/bin/tuat-feed-server

FROM scratch
COPY --from=builder /code/output/bin/tuat-feed-server .

ENV TUAT_FEED_API_ADDR=0.0.0.0:80
EXPOSE 80

CMD ["./tuat-feed-server"]