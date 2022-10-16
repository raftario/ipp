FROM rustlang/rust:nightly-alpine AS builder

WORKDIR /work
RUN cargo new --bin ipp
WORKDIR /work/ipp
COPY Cargo.toml Cargo.lock ./
RUN cargo --locked check --release

COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /work/ipp/target/release/ipp /
CMD ["/ipp"]
