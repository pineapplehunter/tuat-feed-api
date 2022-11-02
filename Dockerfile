FROM rust:1.64-buster as builder
RUN mkdir /code
WORKDIR /code
RUN apt update && apt install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl
COPY Cargo.toml Cargo.lock ./
COPY . ./
RUN cargo install --target x86_64-unknown-linux-musl --path ./server/server/ --root output 
RUN strip /code/output/bin/tuat-feed-server

FROM scratch
COPY --from=builder /code/output/bin/tuat-feed-server .

ENV TUAT_FEED_API_ADDR=0.0.0.0:80
EXPOSE 80

CMD ["./tuat-feed-server"]