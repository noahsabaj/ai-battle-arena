# AI Battle Arena 🎮

A real-time strategy game built specifically for AI agents to play against each other. Unlike traditional games designed for human players, this arena provides direct neural network interfaces, eliminating the need for computer vision or pixel-based interaction.

## Features

- **1000+ TPS** headless simulation for rapid AI training
- **Hexagonal grid** battlefield with strategic positioning
- **Multi-agent teams** (3v3) requiring cooperation
- **Neural communication** between AI teammates
- **Real-time visualization** of AI decision-making
- **Python AI integration** via PyO3

## Current Status

- ✅ Basic Rust/Bevy window rendering
- ✅ Hexagonal grid generation and display
- 🚧 Unit spawning and movement
- 📋 Python AI integration
- 📋 Neural communication system
- 📋 High-performance game loop

## Tech Stack

- **Game Engine**: Rust with Bevy 0.13
- **AI Framework**: Python with PyO3 bindings
- **Graphics**: Bevy's built-in renderer
- **Planned ML**: MAPPO + TarMAC for multi-agent RL

## Getting Started

### Prerequisites

- Rust (latest stable)
- Python 3.10+
- Git

### Running the Game

\`\`\`bash
cargo run
\`\`\`

### Controls

- **Arrow Keys**: Pan camera
- **SPACE**: Start/pause simulation
- **R**: Reset game
- **ESC**: Exit

## Project Structure

\`\`\`
ai_battle_arena/
├── src/
│   ├── main.rs         # Entry point
│   ├── engine/         # Core game loop
│   ├── world/          # Game world (hex grid, units)
│   ├── ai/             # AI integration
│   └── utils/          # Helpers
├── python/             # AI agent implementations
└── assets/             # Game assets
\`\`\`

## Roadmap

- [ ] Complete unit movement system
- [ ] Implement resource gathering
- [ ] Add combat mechanics
- [ ] Integrate Python AI agents
- [ ] Create neural communication protocol
- [ ] Build training framework
- [ ] Add replay system

## License

MIT License

## Acknowledgments

- Built with [Bevy](https://bevyengine.org/)
- Inspired by StarCraft II and OpenAI Five
