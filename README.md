# ChainQuest Idle

MVP Bevy game with idle mechanics, AI-generated maps (torch-rs stub), ENet multiplayer server, and MultiversX SFT scaffolding.

## Build

- Requirements: Rust stable, Linux deps for GLFW/X11 when running Bevy.
- Build all:
```
cargo build --release
```
- Run client:
```
cargo run --bin client
```
- Run server (ENet):
```
cargo run --bin server
```

## Controls
- SPACE: collect resources
- Q: quest placeholder
- M: generate AI map placeholder

## CI/CD
- GitHub Actions workflow .github/workflows/ci.yml builds, tests, audits, and has deploy hooks for Vercel/Render/MultiversX. Configure secrets:
  - VERCEL_TOKEN
  - RENDER_API_KEY, RENDER_SERVICE_ID
  - MX_WALLET_PEM

## Deploy
- Render: use Dockerfile.enet for ENet server. Exposes 8080.
- Vercel: add a frontend later or disable step.
- MultiversX: add SC build/deploy script and address; current SC is a stub.

## Security
- Do not commit wallet.pem. SQLite created locally. Use prepared statements.
