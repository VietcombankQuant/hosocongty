FROM rust:1-alpine3.16 as builder
COPY ./src /src
COPY Cargo.toml /
RUN ["cargo", "build", "--release"]

FROM alpine:3.16
COPY --from=builder /target/release/crawler / 
CMD ["/crawler"]
