# actix-chat

This is a monorepo for backend and frontend Rust applications.

Backend stack:
 * actix
 * diesel

Frontend stack
 * Yew
## How to run

```bash
git clone https://github.com/umbrellait-Grigoriy-Latyshev/actix-chat.git
cd actix-chat
docker-compose up -d
cargo install diesel_cli --no-default-features --features postgres
diesel setup
# compile all
cargo build -r
# run backend in release mode
cargo run -r --bin backend
```

## Developing

VSCode devcontainers supported.

### Backend

Update .env file

```bash
cargo run --bin backend
```

### Frontend

Follow https://yew.rs/docs/tutorial

---

Run project

```bash
cargo run --bin backend
cd frontend
trunk serve --open --proxy-backend=http://localhost:9000/api/
```
