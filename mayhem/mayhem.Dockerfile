# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest AS builder

# Add source code
ADD . /src
WORKDIR /src

# Build fuzzers
RUN cd puffin/fuzz && cargo fuzz build

# Package Stage
FROM ubuntu:latest
COPY --from=builder /src/puffin/fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_* /fuzzers/