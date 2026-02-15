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
- **Exact Monetary Arithmetic**: All financial calculations (prices, fees, share counts, portfolio values) use **scaled integers** (`ScaledInt`) instead of IEEE 754 floating-point (`f64`). This eliminates the rounding errors that silently corrupt balances and P&L figures in most financial software.

## Monetary Precision — Scaled Integer Arithmetic

Financial calculations demand **exact** decimal arithmetic. IEEE 754 floating-point numbers (`f32`/`f64`) cannot represent many common decimal fractions exactly:

```
// Floating-point (f64) — WRONG
0.1 + 0.2 = 0.30000000000000004

// Scaled integer (ScaledInt) — EXACT
0.1 + 0.2 = 0.3
```

These errors compound silently across thousands of transactions, fee calculations, tax computations, and P&L aggregations, eventually producing incorrect balances and audit trails.

### How it works

The `ScaledInt` type (defined in `pmm-utils/src/money.rs`) stores every monetary value as an `i64` multiplied by a fixed scale factor of **10⁸** (100,000,000). This provides **8 decimal digits** of fractional precision — enough for sub-cent accuracy, cryptocurrency amounts, and FX rates.

| Concept          | Representation              |
| ---------------- | --------------------------- |
| `$1.00`          | `ScaledInt(100_000_000)`    |
| `$152.35`        | `ScaledInt(15_235_000_000)` |
| `0.00000001 BTC` | `ScaledInt(1)`              |

- **Addition / Subtraction**: Direct integer add/sub — always exact.
- **Multiplication / Division**: Promoted to `i128` intermediates before rescaling — no overflow, no precision loss.
- **Range**: ±92,233,720,368.54775807 — sufficient for any individual instrument or portfolio roll-up.
- **Serialization**: Serialized as decimal strings (`"152.35"`) in JSON and databases, avoiding any precision loss.

### Usage in the codebase

```rust
use pmm_utils::money::ScaledInt;

let price  = ScaledInt::from_f64(42.37);   // Convert at the boundary (data ingestion)
let shares = ScaledInt::from_f64(150.0);
let fee    = ScaledInt::from_f64(9.99);

let total = (shares * price) + fee;        // Exact: 6365.49
assert_eq!(total.to_f64(), 6365.49);       // Convert back only for display/plotting
```

> **Rule of thumb**: convert from `f64` **once** at data ingestion, perform all intermediate arithmetic with `ScaledInt`, and convert back to `f64` **only** at the output boundary (UI, charts, external API responses).

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
