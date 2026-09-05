# Focus Browser Progress

## Source Spec
agent.md — Focus Browser (codename: focus-browser), 12-week sprint, single-tab, zero IPC, Rust only.

## Current Phase
Phase 1: Foundation (Weeks 1-3) — P4 `styles` active

## Completed Parts
- [x] P1 `window` — Window opens, black egui background, handles close/resize via eframe. Done 2026-09-02.
- [x] P2 `fetch` — HTTP client (`reqwest` + `tokio`). Standalone binary in `parts/p2_fetch/`. Done 2026-09-03.
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
- [x] Skeleton verified — `cargo run` confirms window opens + fetch prints to terminal.
- [ ] ASM1 `static-render` — Integration of P1+P2+P3+P4+P5+P6. Not started; P4 is active.

## Planned Sequence
1. P1 `window` — DONE
2. P2 `fetch` — DONE + INTEGRATED
3. P3 `dom` — DONE standalone
4. P4 `styles` — IN PROGRESS (`stylo` `TNode`/`TElement` for flat `Node`; no CSSOM)
5. P5 `layout` — `taffy` box tree
6. P6 `gpu` — `wgpu` + `cosmic-text`
7. ASM1 `static-render` — Assembly ONLY after P1–P6 pass independently

## What I Did Last Session
- Fixed P3 `push_node` parent init (`u32::MAX`) and `parse_html` output (writes finished `Dom` back to `*dom`).
- Confirmed `progress.md` and `todo.md` updated for P4 active.

## Issues / Blockers
- Blocked: P5 `layout` — waiting on P4 `styles`.
- Blocked: ASM1 — cannot assemble until P4, P5, P6 complete.
- Security / Vulnerability (critical):
  - No `unsafe` FFI unless required by `eframe`/`egui`/`wgpu`.
  - No background processes / IPC.
  - P3 parser must not expose unvalidated HTML to JS until P8 bridge audited.
  - P2 `reqwest` must restrict redirects and validate URLs.
  - P7 `mozjs` context must not expose `eval`/`Function` until sandbox defined.
  - No multi-tab / multi-window.

## Next Session Goal
Implement P4 `styles`: `stylo` `TNode` trait for flat `Node`, compute styles once, no CSSOM, no `getComputedStyle`.

## Notes
- Each part must compile independently (`cargo check` in its directory).
- No integration during part development; assembly is a separate phase (ASM1).
- Window layer is eframe/egui; winit was never used.
- P2 fetch is integrated into skeleton; do not wire into `src/main.rs` again until ASM1.
- All updates noted in `progress.md`; all plans/doing/finished noted in `todo.md`.
- `reqwest` uses `rustls-tls` (no OpenSSL).
- GPU-first: `IntegratedGpu` adapter selected; CPU (`llvmpipe`) available only for non-GPU parts.
- Multi-threaded: `tokio` runtime uses multi-threaded scheduler by default.
- P3 DOM is greenfield: no `Rc`, no `RefCell`, no `Box`, no `Arc`, no trait objects, no vtables, no per-node heap allocation. Flat array + CSR + arenas.
- `mozjs` (SpiderMonkey) is the JS engine for P7+; `boa_engine` is not used.
