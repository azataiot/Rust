ARG ARCH_MUSL=x86_64-musl
ARG TARGET_TRIPLE=x86_64-unknown-linux-musl

FROM --platform=$BUILDPLATFORM messense/rust-musl-cross:${ARCH_MUSL} AS builder
ARG TARGET_TRIPLE
WORKDIR /app

ENV SQLX_OFFLINE=true
ENV RUSTFLAGS="-C strip=symbols"

COPY . .

ENV RUSTFLAGS="-C strip=symbols"
RUN cargo build --release --target ${TARGET_TRIPLE}

FROM scratch
WORKDIR /app
ARG TARGET_TRIPLE
COPY --from=builder /app/target/${TARGET_TRIPLE}/release/rust-musl-cross /app/rust-musl-cross
USER 1000:1000
EXPOSE 8000
ENV RUST_LOG=info
ENTRYPOINT ["/app/rust-musl-cross"]
