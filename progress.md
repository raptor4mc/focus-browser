# Focus Browser Progress

## Source Spec
agent.md — Focus Browser (codename: focus-browser), 12-week sprint, single-tab, zero IPC, Rust only.

## Current Phase
Phase 1: Foundation (Weeks 1-3)

## Completed Parts
- [x] P1 `window` — Window opens, black egui background, handles close/resize via eframe. Done 2026-09-02.
  - File: `parts/p1_window/src/main.rs`
  - File: `parts/p1_window/Cargo.toml`
  - File: `src/main.rs` (assembly link, human-review only)
  - File: `Cargo.toml` (eframe 0.30 + egui 0.30, wgpu feature)
  - Verified standalone: `eframe` event loop runs; window renders black background; close/resize handled by viewport; Vulkan forced via `WGPU_BACKEND=vulkan`; adapter list shows `Virtio-GPU` (GPU) and `llvmpipe` (CPU).
  - Security: No network, no script execution, no external input parsing. Window creation is isolated. No `unsafe` except required `set_var`. We never used winit.

## Current Part
P2 `fetch` — HTTP client (`reqwest` + `tokio`). Standalone binary in `parts/p2_fetch/`.

## Planned Sequence (user instruction + agent.md)
1. P1 `window` — DONE (eframe/egui, Vulkan forced)
2. P2 `fetch` — IN PROGRESS (reqwest + tokio)
3. P3 `dom` — HTML parsing (`html5ever`) + DOM tree (`Rc<Node>`)
4. P4 `styles` — CSS / `stylo` traits (`TNode`, `TElement`)
5. P2 `fetch` — HTTP (`reqwest` + `tokio`) — already planned above; sequence per agent.md is P2 before P3/P4 assembly.
6. P5 `layout` — `taffy` box tree
7. P6 `gpu` — `wgpu` + `cosmic-text`
8. ASM1 `static-render` — Assembly ONLY after P1–P6 pass independently

Note: User specified window → html → css → dom. We treat P3 as covering both HTML parsing and DOM tree construction; CSS (P4) is done before full DOM link verification to match the requested order.

## What I Did Last Session
- Replaced `winit` with `eframe` + `egui` in root `Cargo.toml` and `parts/p1_window/Cargo.toml`.
- Migrated `src/main.rs` and `parts/p1_window/src/main.rs` to `eframe::App` / `run_native` API.
- Added black `egui` background via `CentralPanel` + `rect_filled`.
- Updated `agent.md` to state we never used winit and always used eframe/egui.
- Forced Vulkan specifically: `unsafe { std::env::set_var("WGPU_BACKEND", "vulkan") }`, `Backends::VULKAN`, `NativeOptions::default()`.
- Added adapter enumeration to terminal to confirm GPU vs CPU.
- Created isolated P2 `fetch` part (`parts/p2_fetch/`).

## Issues / Blockers
- None for P1.
- Blocked: P3 `dom` — need to decide `Rc<RefCell<Node>>` vs arena (`indextree`) for parent/child links.
- Blocked: P4 `styles` — `stylo` `TNode` trait requires P3 node type definition.
- Security / Vulnerability (critical — browsers are high-risk):
  - No `unsafe` FFI unless absolutely required by `eframe`/`egui`/`wgpu`.
  - No background processes / IPC (single-process rule).
  - P3 parser must not expose unvalidated HTML to JS until P8 bridge is audited.
  - P2 `reqwest` must restrict redirects and validate URLs before fetch.
  - P7 `boa_engine` must not expose `eval`/`Function` until sandbox is defined.
  - No multi-tab / multi-window (scope is one tab).

## Next Session Goal
Verify P2 `fetch`: `cargo check` in `parts/p2_fetch/`, run binary, confirm `https://example.com` fetches and prints status/length. Do not integrate with P1.

## Notes
- Each part must compile independently (`cargo check` in its directory).
- No integration during part development; assembly is a separate phase (ASM1).
- If `cargo check` fails >3 prompts, add to `todo.md` and simplify.
- Human review required before touching `src/main.rs` or `src/engine/js.rs`.
- After fixing `src/main.rs`, run `cargo clean && cargo run` to clear stale build artifacts.
- Window layer is eframe/egui; winit was never used.
- P2 fetch is standalone; do not wire into `src/main.rs` until ASM1.
