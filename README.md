# dafer-quant

> **⚠️ Work In Progress — This project is under active development and is NOT functional yet.** Many core features are missing, partially implemented, or subject to significant changes. APIs, data structures, and module interfaces may change without notice. Contributions and feedback are welcome, see [Contributing](#contributing) below, but please be aware that this is an early-stage project.

---

## Overview

**dafer-quant** is a high-performance quantitative finance framework built entirely in **Rust**, designed to leverage the language's core strengths — **memory safety without garbage collection**, **zero-cost abstractions**, and **fearless concurrency** — to deliver a robust and efficient platform for portfolio management and quantitative analysis.

The framework is architected as a modular Rust workspace, prioritizing:

- **Performance**: Compiled to native code with aggressive optimizations (`opt-level = 3`, LTO, `panic = abort` in release). Data processing is powered by [Polars](https://pola.rs/), a blazing-fast DataFrame library built on Apache Arrow, enabling columnar data operations at speeds that rival or exceed C/C++ implementations.
- **Safety**: Rust's ownership model and type system eliminate entire classes of bugs — no null pointer dereferences, no data races, no use-after-free. Every data pipeline and concurrent operation is verified at compile time.
- **Zero-Cost Abstractions**: Generic programming, traits, and procedural macros (see `pmm-perf`) provide high-level ergonomics with no runtime overhead.
- **Concurrency**: Asynchronous I/O via [Tokio](https://tokio.rs/) for database operations and network calls, with Rust's borrow checker guaranteeing thread safety at compile time.

## Architecture

The project is organized as a Cargo workspace with four specialized crates:

```
dafer-quant/
├── pmm-core        # Core business logic: data ingestion, portfolio modeling, database layer
├── pmm-perf        # Procedural macro crate for compile-time performance instrumentation
├── pmm-utils       # Shared utilities: data processing helpers, plotting (Plotly)
└── pmm-frontend    # Web frontend built with Dioxus (Rust-native reactive UI framework)
```

### `pmm-core`
The backbone of the framework. Handles:
- **Data ingestion** — CSV parsing via Polars LazyFrames for memory-efficient, streaming data processing
- **Portfolio modeling** — Owners, Assets, Positions, and Transactions as strongly-typed domain models
- **Database layer** — Async SQLite via SQLx with compile-time query verification, connection pooling, and foreign key enforcement
- **Security** — Password hashing with Argon2 (memory-hard KDF)

### `pmm-perf`
A custom **procedural macro crate** (`#[performance_log]`) that injects timing instrumentation into any function at compile time. Supports configurable output modes (`print`, `file`, `both`, `none`) with zero overhead when disabled — a true zero-cost abstraction.

### `pmm-utils`
Shared utilities including data processing helpers and interactive visualization via [Plotly.rs](https://github.com/plotly/plotly.rs) for generating financial charts and histograms.

### `pmm-frontend`
A reactive web UI built with [Dioxus](https://dioxuslabs.com/), a Rust-native framework inspired by React. Compiles to WebAssembly for browser deployment, with support for desktop and mobile targets.

## Build Profiles

The project is configured with carefully tuned build profiles:

| Profile   | Optimization | Debug Info | LTO     | Incremental | Codegen Units        |
| --------- | ------------ | ---------- | ------- | ----------- | -------------------- |
| `dev`     | Off (`0`)    | Full       | Off     | Yes         | 16 (fast builds)     |
| `release` | Max (`3`)    | Off        | Fat LTO | No          | 1 (max optimization) |

## Getting Started

> **Note**: This project requires Rust Edition 2024. Make sure you have the latest stable Rust toolchain installed.

```bash
# Clone the repository
git clone https://github.com/dafer238/dafer-quant.git
cd dafer-quant

# Build all crates
cargo build

# Run the core application
cargo run -p pmm-core

# Run tests
cargo test --workspace

# Build for release (full optimizations)
cargo build --release
```

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable, Edition 2024 support)
- SQLite3 development libraries

## What's Missing / Planned

This is an early-stage project. Major areas still to be implemented:

- [ ] Real-time market data integration (Alpha Vantage, Twelve Data, Finnhub)
- [ ] Portfolio analytics and performance metrics (Sharpe, Sortino, drawdown analysis)
- [ ] Risk modeling (VaR, Monte Carlo simulations)
- [ ] Full frontend implementation with interactive dashboards
- [ ] REST API layer for external integrations
- [ ] Backtesting engine
- [ ] Multi-currency support and FX handling
- [ ] Comprehensive documentation and examples
- [ ] CI/CD pipeline
- [ ] Published crate on crates.io

## Open Source

This project is proudly **open source** and licensed under the [**GNU General Public License v3.0 (GPL-3.0)**](LICENSE).

This means you are free to:
- **Use** this software for any purpose
- **Study** how it works and adapt it to your needs
- **Share** copies with others
- **Improve** the program and release your improvements to the public

Under the condition that any derivative work must also be distributed under the same GPL-3.0 license, ensuring that the software and all its derivatives remain free and open.

## Contributing

Contributions are welcome! Since the project is in its early stages, please open an issue first to discuss what you'd like to work on before submitting a pull request. This helps avoid duplicated effort and ensures alignment with the project's direction.

## Contact

**Daniel Perez** — [daniel.perez@pmmgt.com](mailto:daniel.perez@pmmgt.com)

## License

This project is licensed under the GNU General Public License v3.0 — see the [LICENSE](LICENSE) file for details.
