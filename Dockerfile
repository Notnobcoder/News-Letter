FROM rust:1.89.0-bookworm AS build
WORKDIR /app
COPY . .
ENV RUST_BACKTRACE=1
RUN rm -rf target Cargo.lock && cargo build --release

# Use cc flavor (includes runtime libs like libgcc)
FROM gcr.io/distroless/cc-debian12
COPY --from=build /app/target/release/news-letter /server

EXPOSE 8000
CMD ["/server"]
