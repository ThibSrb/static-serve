FROM rust:latest as builder
WORKDIR /usr/src/static-serve
COPY . .
RUN cargo install --path .

FROM debian:stable-slim
RUN apt-get update && apt-get install -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/static-serve /usr/local/bin/static-serve
RUN mkdir /data
WORKDIR /data
EXPOSE 3000
CMD ["static-serve"]
