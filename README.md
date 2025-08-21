```txt
 ███████╗██╗         ██████╗ ██╗██╗  ██╗███████╗██╗         ███████╗ ██████╗ ██████╗███╗   ███╗███████╗
 ██╔════╝██║         ██╔══██╗██║██║ ██╔╝██╔════╝██║         ██╔════╝██╔═══██╗██╔════╝████╗ ████║██╔════╝
 █████╗  ██║         ██████╔╝██║█████╔╝ █████╗  ██║         ███████╗██║   ██║██║     ██╔████╔██║█████╗  
 ██╔══╝  ██║         ██╔══██╗██║██╔═██╗ ██╔══╝  ██║         ╚════██║██║   ██║██║     ██║╚██╔╝██║██╔══╝  
 ██║     ███████╗    ██║  ██║██║██║  ██╗███████╗███████╗    ███████║╚██████╔╝╚██████╗██║ ╚═╝ ██║███████╗
 ╚═╝     ╚══════╝    ╚═╝  ╚═╝╚═╝╚═╝  ╚═╝╚══════╝╚══════╝    ╚══════╝ ╚═════╝  ╚═════╝╚═╝     ╚═╝╚══════╝
                                             Pixel‑Perfect Forms — README.md
```

# Pixel‑Perfect Forms

Rust + WebAssembly app that recreates California Judicial Council forms as pixel‑perfect, verifiable, screen‑first and print‑accurate clones.

## Overview

Built with Yew + Trunk for the UI and a Rust/WASM core. Uses modern web APIs (Houdini, OffscreenCanvas, Popover, Container Queries) and calibration to achieve sub‑millimeter fidelity.

## Quickstart

Prereqs:
- Rust stable + rustup
- wasm32 target: `rustup target add wasm32-unknown-unknown`
- Trunk: `cargo install trunk`

Run locally:

```bash
trunk serve --config ./app/Trunk.toml --open
# or simply from ./app since index.html is there
cd app && trunk serve --open
```

Build:

```bash
cd app
trunk build --release
```

## Project Structure

- `Cargo.toml` (workspace)
- `app/`
  - `Cargo.toml` — Yew app
  - `index.html` — Trunk entry (links Rust and copies `static/`)
  - `src/main.rs` — Yew `App`
  - `static/` — Worklets/Workers
    - `bootstrap.js` — progressive init
    - `worklets/paint-grid.js` — Houdini grid painter
    - `workers/diff-worker.js` — placeholder worker

## Documentation

- Research: `docs/context-engineering/research-findings/RESEARCH.md`
- Plans: `PLAN.md`

## Technology Targets (2025)

- Container Queries, Popover, OffscreenCanvas, Wasm SIMD: baseline
- Anchor Positioning, View Transitions, Houdini Paint: progressive
- WebGPU, Wasm Threads (COOP+COEP): opt‑in

## License

TBD

---

Built for verifiable, accessible, and accurate legal forms.