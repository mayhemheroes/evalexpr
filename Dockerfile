FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /evalexpr
WORKDIR /evalexpr/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /evalexpr/fuzz/target/x86_64-unknown-linux-gnu/release/evalexpr-fuzz /