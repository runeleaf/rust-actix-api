# Rust Actix API Example

## Setup

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Dependencies

1. actix-web
2. postgresql

```
cargo install cargo-watch
cargo watch -x run
```

```
cargo install cargo-edit
cargo add actix-web
```

```
rustup component add rustfmt
cargo fmt
```

### deploy
render.com

## usage


#### database setup

```
cargo install diesel_cli --no-default-features --features postgres
diesel setup
diesel migration generate create_messages
diesel migration run
# diesel migration redo
```

#### start web server
```
cp .env.example .env
make up
```
