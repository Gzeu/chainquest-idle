# ChainQuest Idle

🎮 **Blockchain RPG Idle Game** cu SFT-uri MultiversX, multiplayer co-op și AI generative maps.

## 🚀 Funcționalități Implementate

### ✅ Core Game Systems
- **Quest System**: Generare dinamică de quest-uri cu difficulty scaling și SFT rewards
- **AI Map Generation**: torch-rs integration cu fallback procedural pentru hărți 16x16
- **Idle Progression**: Sistem ECS cu resource collection și level-up automat
- **Security & Anti-cheat**: Rate limiting, input sanitization, suspicious activity detection

### ✅ Multiplayer & Network
- **ENet Integration**: Server/client architecture cu compression și rate limiting (10 packets/sec)
- **Real-time Communication**: Broadcast messaging, peer management, network statistics
- **Co-op Support**: 2-4 jucători simultani cu sincronizare de state

### ✅ Blockchain Integration
- **MultiversX SFT**: Smart contract scaffolding pentru minting și staking
- **Wallet Integration**: Pregătit pentru xPortal connection
- **Testnet Ready**: Deploy scripts pentru MultiversX testnet

### ✅ Infrastructure
- **Bevy ECS**: Architecture modulară cu components, resources, systems
- **CI/CD Pipeline**: GitHub Actions pentru build, test, deploy
- **Multi-platform Deploy**: Vercel (frontend), Render (server), WASM (browser)

## 🧭 Frontend Next.js

- **Structură**: `frontend/app` cu rutele `/wallet`, `/wasm`, `/status`[1]
- **Wallet**: Integrare `@multiversx/sdk-dapp` (testnet) cu buton de login pe `/wallet`[1]
- **WASM**: Placeholder pe `/wasm` cu canvas; urmează integrarea Bevy WebAssembly[1]
- **Status/Deploy**: Ghid în `/status` pentru Vercel/Render și comenzi locale[1]
- **Comenzi**:
  ```bash
  cd frontend
  npm install
  npm run dev
  ```
- **Vercel env**: setează `NEXT_PUBLIC_WC_PROJECT_ID` (WalletConnect V2 Project ID) înainte de deploy.

## 🎯 Controls

- **SPACE**: Collect resources manually
- **Q**: Complete active quest
- **M**: Generate new AI map (16x16 grid)

## 🔧 Build & Run

### Requirements
- Rust stable (1.70+)
- Linux: `sudo apt install libx11-dev libxcursor-dev libxrandr-dev libxinerama-dev libxi-dev libgl1-mesa-dev`
- Windows/Mac: No additional dependencies

### Build Commands
```bash
# Build all targets
cargo build --release

# Run client (interactive game)
cargo run --bin client

# Run server (ENet multiplayer)
cargo run --bin server

# Run tests
cargo test

# Security audit
cargo audit
```

### Environment Configuration
Copy `.env.example` to `.env` and configure:
```env
CQ_HOST=0.0.0.0
CQ_PORT=8080
```

## 🌐 Deployment

### Render (ENet Server)
```bash
# Uses Dockerfile.enet
# Exposes port 8080
# Auto-deploy on main branch push
```

### Vercel (Frontend)
```bash
# Next.js app în /frontend
# WASM integration pentru browser client
# xPortal wallet connection (NEXT_PUBLIC_WC_PROJECT_ID necesar)
```

### MultiversX Testnet
```bash
# Smart contracts în /sc
# SFT minting și staking
# Configurează secretul MX_WALLET_PEM
```

## 🏗️ Architecture

### Core Systems
```
src/
├── quest_system.rs         # Dynamic quest generation & completion
├── ai/map_generator.rs     # AI-powered map generation (torch-rs)
├── security/mod.rs         # Anti-cheat & input validation
├── multiplayer/network.rs  # ENet with rate limiting & compression
├── components.rs           # ECS components (Player, Quest, SFT, etc.)
├── systems_idle.rs         # Idle progression mechanics
└── game_plugin.rs          # Main Bevy plugin integration
```

### Dependencies
- **Bevy 0.12**: Game engine cu ECS
- **torch-rs**: AI model inference pentru map generation
- **ENet 1.3**: Low-latency UDP networking
- **MultiversX SDK**: Blockchain integration
- **flate2**: Network packet compression
- **parking_lot**: Thread-safe collections
- **Next.js 14** + **@multiversx/sdk-dapp** + **zustand** (frontend)

## 📊 Development Status

**Progres MVP: ~60%** (25% → 60% cu implementările recente)

### ✅ Completate
- [x] Repository setup și CI/CD
- [x] Bevy ECS architecture
- [x] Quest system complet
- [x] AI map generation cu torch-rs
- [x] Security & anti-cheat
- [x] Enhanced networking
- [x] Multiplayer foundation
- [x] Frontend Next.js scaffolding cu wallet/wasm/status

### 🔄 În Progres
- [ ] Smart contract deployment
- [ ] WASM Bevy integration completă în /wasm
- [ ] Frontend inventory SFT (după SC deploy)
- [ ] Production testing

### 📋 Următoarele Priorități
1. **SC Deploy** (2h): Smart contract pe testnet
2. **Frontend Integration** (3h): Next.js + WASM + xPortal
3. **Production Deploy** (2h): Render + Vercel deployment
4. **E2E Testing** (2h): Full multiplayer testing

## 🔒 Security Features

- **Rate Limiting**: 10 packets/sec per client
- **Input Sanitization**: Regex validation pentru usernames/messages
- **Anti-cheat**: Suspicious activity detection
- **Secure Storage**: SQLite cu prepared statements
- **Network Compression**: Gzip pentru packets >100 bytes

## 🎮 Game Features

### Quest System
- 4 difficulty levels: Easy → Epic
- Dynamic rewards scaling cu player level
- SFT rewards pentru Hard/Epic quests
- Auto-completion și manual triggers

### AI Map Generation
- torch-rs neural network inference
- Procedural fallback dacă modelul lipsește
- Biome generation (Forest, Desert, Mountains, Swamp)
- Structured placement (quests în centru, portals pe margini)

### Multiplayer
- 2-4 jucători co-op
- Real-time resource sharing
- Collaborative quest completion
- Network statistics și monitoring

## 📈 Performance

- **Target**: 60 FPS client, <50ms network latency
- **Memory**: <100MB runtime footprint
- **Network**: <1KB/sec per player în idle
- **AI Model**: <100MB pentru deployment

## 🤝 Contributing

1. Fork repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push branch: `git push origin feature/amazing-feature`
5. Open Pull Request

## 📜 License

MIT License - vezi [LICENSE](LICENSE) pentru detalii.

## 🔗 Links

- **Repository**: https://github.com/Gzeu/chainquest-idle
- **MultiversX**: https://multiversx.com/
- **Bevy Engine**: https://bevyengine.org/
- **Render**: https://render.com/
- **Vercel**: https://vercel.com/

---

**ChainQuest Idle** - Unde blockchain-ul întâlnește gaming-ul! 🎯⚔️