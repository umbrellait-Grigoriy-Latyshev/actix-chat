# actix-chat


## How to run

```bash
git clone https://github.com/umbrellait-Grigoriy-Latyshev/actix-chat.git
cd actix-chat
docker-compose up -d
cargo install diesel_cli --no-default-features --features postgres
diesel setup
# run in release mode
cargo run -r 
```

## Developing

VSCode devcontainers supported.

It's possible to use `cargo watch ...` to reload server after changes:

```bash
cargo install cargo-watch
cargo watch -x "run --"
```
