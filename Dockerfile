# env development
FROM rust:1.60.0-slim as development
WORKDIR /opt/app
RUN apt update -qq && apt install -y libssl-dev build-essential libpq-dev vim apt-transport-https
RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres
RUN rustup component add rustfmt
COPY . .

# env build
FROM development as build
RUN cargo build --release

# env production
FROM rust:1.60.0-slim as production
RUN apt update -qq && apt install -y build-essential libpq-dev apt-transport-https
COPY --from=build /app/target/release/rust-actix-api .
EXPOSE 3000
CMD ["./rust-actix-api"]
