FROM rust:1.64-buster as builder
RUN mkdir /code
WORKDIR /code
COPY . ./
RUN cargo install --path ./server/server/ --root output 

FROM debian:buster-slim
RUN apt-get update && \
    apt-get install -y curl && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /code/output/bin/tuat-feed-server .

ENV TUAT_FEED_API_ADDR=0.0.0.0:80
EXPOSE 80

CMD ["/bin/bash", "-c", "./tuat-feed-server"]