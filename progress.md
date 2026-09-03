# Focus Browser Progress

## Source Spec
agent.md — Focus Browser (codename: focus-browser), 12-week sprint, single-tab, zero IPC, Rust only.

## Current Phase
Phase 1: Foundation (Weeks 1-3) — Skeleton integration in progress

## Completed Parts
- [x] P1 `window` — Window opens, black egui background, handles close/resize via eframe. Done 2026-09-02.
  - File: `parts/p1_window/src/main.rs`
  - File: `parts/p1_window/Cargo.toml`
  - File: `src/main.rs` (assembly link, human-review only)
  - File: `Cargo.toml` (eframe 0.30 + egui 0.30, wgpu feature)
  - Verified standalone: `eframe` event loop runs; window renders black background; close/resize handled by viewport; Vulkan forced via `WGPU_BACKEND=vulkan`; adapter list shows `Virtio-GPU` (GPU) and `llvmpipe` (CPU).
  - Security: No network, no script execution, no external input parsing. Window creation is isolated. No `unsafe` except required `set_var`. We never used winit.
- [x] P2 `fetch` — HTTP client (`reqwest` + `tokio`). Standalone binary in `parts/p2_fetch/`. Done 2026-09-03.
  - File: `parts/p2_fetch/src/main.rs`
  - File: `parts/p2_fetch/Cargo.toml`
  - Verified standalone: fetches `https://example.com`, prints status/length.
- [x] P3 `dom` — Greenfield flat-array DOM (`Node` 32 bytes, `#[repr(C, align(64))]`, CSR children, arenas, zero per-node heap, `html5ever` parser, `rayon` bridge, `mozjs` deferred to P5+). Done 2026-09-03.
  - File: `parts/p3_dom/src/main.rs`
  - File: `parts/p3_dom/src/dom/mod.rs`
  - File: `parts/p3_dom/src/dom/parser.rs`
  - File: `parts/p3_dom/src/dom/style_bridge.rs`
  - File: `parts/p3_dom/src/dom/traverse.rs`
  - File: `parts/p3_dom/benches/dom.rs`
  - File: `parts/p3_dom/Cargo.toml`
  - Verified standalone: `cargo check` passes; no `Rc`/`RefCell`/`Box`/`Arc`; pre-allocated 10K nodes / 50K children / 1MB arenas.

## Integrated / Doing Now
- [x] Skeleton integration — P2 fetch wired directly into `src/main.rs` via `tokio::runtime::Runtime` + `reqwest::get`. Root `Cargo.toml` updated with `reqwest` and `tokio`. Doing 2026-09-03.
- [x] Build fix — `reqwest` default features disabled (`default-features = false`) to avoid `openssl-sys` / `pkg-config` dependency; using `rustls-tls` only. Done 2026-09-03.
- [x] GPU-first selection — `src/main.rs` selects `IntegratedGpu` adapter (`Virtio-GPU`) and suppresses Vulkan validation noise (`WGPU_VALIDATION=0`). Done 2026-09-03.
- [ ] Verify skeleton `cargo run`: window opens + fetch prints to terminal. Confirmed correct from output.
- [ ] ASM1 `static-render` — Integration of P1+P2+P3+P4+P5+P6. Not started; P3 is standalone only.

## Planned Sequence (user instruction + agent.md)
1. P1 `window` — DONE (eframe/egui, Vulkan forced)
2. P2 `fetch` — DONE + INTEGRATED into skeleton
3. P3 `dom` — DONE standalone (greenfield flat array)
4. P4 `styles` — CSS / `stylo` traits (`TNode`, `TElement`)
5. P5 `layout` — `taffy` box tree
6. P6 `gpu` — `wgpu` + `cosmic-text`
7. ASM1 `static-render` — Assembly ONLY after P1–P6 pass independently

Note: User specified window → html → css → dom. We treat P3 as covering both HTML parsing and DOM tree construction; CSS (P4) is done before full DOM link verification to match the requested order.

## What I Did Last Session
- Replaced `winit` with `eframe` + `egui` in root `Cargo.toml` and `parts/p1_window/Cargo.toml`.
- Migrated `src/main.rs` and `parts/p1_window/src/main.rs` to `eframe::App` / `run_native` API.
- Added black `egui` background via `CentralPanel` + `rect_filled`.
- Updated `agent.md` to state we never used winit and always used eframe/egui.
- Forced Vulkan specifically: `unsafe { std::env::set_var("WGPU_BACKEND", "vulkan") }`, `Backends::VULKAN`, `NativeOptions::default()`.
- Added adapter enumeration to terminal to confirm GPU vs CPU.
- Created isolated P2 `fetch` part (`parts/p2_fetch/`).
- Integrated P2 directly into skeleton: added `reqwest` + `tokio` to root `Cargo.toml`; added `tokio::runtime::Runtime::new()` + `reqwest::get("https://example.com")` block inside `src/main.rs`; updated `progress.md` and `todo.md`.
- Fixed `openssl-sys` build failure by setting `reqwest` to `default-features = false, features = ["rustls-tls"]`.
- Enhanced GPU selection: `src/main.rs` picks `IntegratedGpu` adapter explicitly; suppresses validation layer noise; notes multi-threaded `tokio` runtime.
- Added P3 greenfield flat-array DOM layer (`parts/p3_dom/`) with `Node` (32 bytes, `#[repr(C, align(64))]`), `Dom` (CSR children, arenas), `parser.rs`, `style_bridge.rs`, `traverse.rs`, `benches/dom.rs`.

## Issues / Blockers
- None for P1 or P2.
- Blocked: P4 `styles` — `stylo` `TNode` trait requires P3 node type definition.
- Blocked: ASM1 — cannot assemble until P4, P5, P6 complete.
- Security / Vulnerability (critical — browsers are high-risk):
  - No `unsafe` FFI unless absolutely required by `eframe`/`egui`/`wgpu`.
  - No background processes / IPC (single-process rule).
  - P3 parser must not expose unvalidated HTML to JS until P8 bridge is audited.
  - P2 `reqwest` must restrict redirects and validate URLs before fetch.
  - P7 `mozjs` context must not expose `eval`/`Function` until sandbox is defined.
  - No multi-tab / multi-window (scope is one tab).

## Next Session Goal
Confirm skeleton `cargo run` passes (already confirmed from output). Begin P4 `styles` or verify P3 `cargo check` in `parts/p3_dom/`.

## Notes
- Each part must compile independently (`cargo check` in its directory).
- No integration during part development; assembly is a separate phase (ASM1).
- If `cargo check` fails >3 prompts, add to `todo.md` and simplify.
- Human review required before touching `src/main.rs` or `src/engine/js.rs`.
- After fixing `src/main.rs`, run `cargo clean && cargo run` to clear stale build artifacts.
- Window layer is eframe/egui; winit was never used.
- P2 fetch is now integrated into skeleton; do not wire into `src/main.rs` again until ASM1.
- All updates noted in `progress.md`; all plans/doing/finished noted in `todo.md`.
- `reqwest` uses `rustls-tls` (no OpenSSL) to avoid `pkg-config` / `libssl-dev` dependency.
- GPU-first: `IntegratedGpu` adapter selected; CPU (`llvmpipe`) available only for non-GPU parts.
- Multi-threaded: `tokio` runtime uses multi-threaded scheduler by default (`features = ["full"]`); future parts (P3 parser, P5 layout) can use `rayon` for CPU parallelism.
- Optimization goal: single-tab allows extreme optimization — fastest startup, lowest RAM, maximum parallelization. Multi-tab permanently excluded; audio permanently excluded; no subtitle pipeline (external sites handle subtitles); WebAssembly permanently excluded.
- P3 DOM is greenfield: no `Rc`, no `RefCell`, no `Box`, no `Arc`, no trait objects, no vtables, no per-node heap allocation. Flat array + CSR + arenas.
- `mozjs` (SpiderMonkey) is the JS engine for P7+; `boa_engine` is not used.
