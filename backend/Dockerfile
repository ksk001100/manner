# 開発環境
FROM rust:1.49-alpine as develop-stage
WORKDIR /app
RUN apk add --no-cache musl-dev
RUN cargo install cargo-watch
COPY . .

# ビルド環境
FROM develop-stage as build-stage
RUN cargo build --release

# 本番環境
FROM rust:1.49-alpine
COPY --from=build-stage /app/target/release/api .
EXPOSE 8088
CMD ["/usr/local/bin/api"]
