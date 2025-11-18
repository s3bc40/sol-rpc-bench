# ⚡ Solana RPC Bench

A modest TUI (Terminal User Interface) for benchmarking Solana RPC endpoints in real-time. Built with Rust and Ratatui.

![Solana RPC Bench Demo](./assets/sol-rpc-bench-demo.gif)

## 🚀 Features

- **Real-time benchmarking** of 11+ public Solana RPC endpoints
- **Beautiful TUI** with live-updating bar charts and tables
- **Color-coded latency** indicators (Green < 200ms, Yellow < 500ms, Red ≥ 500ms)
- **Parallel benchmarking** - tests all endpoints simultaneously
- **Auto-refresh** - updates results every ~3-8 seconds
- **Lightweight** - pure Rust, no external dependencies beyond HTTP client

## 📊 What It Does

Solana RPC Bench continuously pings public Solana RPC endpoints using the `getVersion` JSON-RPC method, measuring:

- **Latency** (response time in milliseconds)
- **Health status** (whether the endpoint is responding)
- **Ranking** (fastest to slowest, healthy endpoints first)

Perfect for:

- 🔍 Finding the fastest public RPC for your app
- 📈 Monitoring RPC performance over time
- 🛠️ Comparing different RPC providers
- 🎓 Learning Rust + Ratatui

## 🎯 Quick Start

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))

### Installation

```bash
# Clone the repository
git clone https://github.com/s3bc40/sol-rpc-bench.git
cd sol-rpc-bench

# Run the benchmarker
cargo run --release
```

### Usage

```bash
# Start benchmarking
cargo run --release

# Press 'q' to quit
```

## 📸 Screenshots

### Bar Chart View

Shows the top 11 fastest RPCs with color-coded latency bars.

### Results Table

Displays all endpoints ranked by speed, with health status indicators.

## 🏗️ Architecture

```
src/
├── main.rs      # Entry point, terminal setup, event loop
├── app.rs       # Application state management
├── rpc.rs       # RPC benchmarking logic
└── ui.rs        # Ratatui rendering (charts, tables, help bar)
```

### Key Technologies

- **[Tokio](https://tokio.rs/)** - Async runtime for parallel benchmarking
- **[Ratatui](https://ratatui.rs/)** - Terminal UI framework
- **[Crossterm](https://github.com/crossterm-rs/crossterm)** - Cross-platform terminal manipulation
- **[Reqwest](https://github.com/seanmonstar/reqwest)** - HTTP client for RPC calls

## 🔧 Configuration

### Tested RPC Endpoints

The tool currently benchmarks these public Solana RPC endpoints:

- Solana Mainnet Beta (official)
- Ankr
- Helius
- PublicNode
- Chainstack
- Nodies
- Triton
- Magic Eden
- Shyft
- GenesysGo
- Rpcpool

_Want to add more?_ Edit [`src/rpc.rs`](src/rpc.rs) and add to the `get_rpc_list()` function.

### Timeout Settings

Default timeout: **5 seconds** per endpoint

Change it in [`src/rpc.rs`](src/rpc.rs):

```rust
let response = tokio::time::timeout(
    Duration::from_secs(5),  // <-- Change this
    client.post(url).json(&request_body).send()
).await;
```

## 🎨 Color Legend

- 🟢 **Green** - Latency < 200ms (Excellent)
- 🟡 **Yellow** - Latency 200-500ms (Good)
- 🔴 **Red** - Latency > 500ms or unhealthy (Poor)

## 🧪 Development

### Run in debug mode

```bash
cargo run
```

### Run tests

```bash
cargo test
```

### Build optimized release

```bash
cargo build --release
./target/release/sol-rpc-bench
```

## 🗺️ Roadmap (V2)

- [ ] Add private RPC support (via config file or env vars)
- [ ] Export results to CSV/JSON
- [ ] Historical latency graphs
- [ ] Customizable refresh intervals
- [ ] More RPC methods (getHealth, getSlot, etc.)
- [ ] Configurable endpoint list via TOML
- [ ] Unit tests for core logic

## 🤝 Contributing

Contributions welcome! Feel free to:

- Report bugs
- Suggest features
- Submit pull requests

## 📝 License

MIT License - see [LICENSE](LICENSE) for details

## 🙏 Acknowledgments

- Built with ❤️ using [Ratatui](https://ratatui.rs/)
- Inspired by the need for reliable Solana RPC endpoints
- Special thanks to all public RPC providers

## 📬 Contact

**s3bc40** - [@s3bc40](https://twitter.com/s3bc40)

Project Link: [https://github.com/s3bc40/sol-rpc-bench](https://github.com/s3bc40/sol-rpc-bench)

---

⭐ **Star this repo** if you find it useful!
