# syntax=docker/dockerfile:1
# build stage
FROM rust:latest AS builder
WORKDIR /app/
COPY . .
RUN cargo build --release --bin scheduler
# final image
FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /bin/
COPY --from=builder /app/target/release/scheduler app
ENTRYPOINT ["/bin/app"]
