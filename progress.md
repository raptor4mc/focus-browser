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
  - File: `Cargo.toml` (eframe 0.30 + egui 0.30)
  - Verified standalone: `eframe` event loop runs; window renders black background; close/resize handled by viewport.
  - Security: No network, no script execution, no external input parsing. Window creation is isolated. No `unsafe`. We never used winit.

## Current Part
P1 `window` — Complete (eframe/egui). Ready to proceed to P3 `dom`.

## Planned Sequence (user instruction + agent.md)
1. P1 `window` — DONE (eframe/egui)
2. P3 `dom` — HTML parsing (`html5ever`) + DOM tree (`Rc<Node>`)
3. P4 `styles` — CSS / `stylo` traits (`TNode`, `TElement`)
4. P2 `fetch` — HTTP (`reqwest` + `tokio`)
5. P5 `layout` — `taffy` box tree
6. P6 `gpu` — `wgpu` + `cosmic-text`
7. ASM1 `static-render` — Assembly ONLY after P1–P6 pass independently

Note: User specified window → html → css → dom. We treat P3 as covering both HTML parsing and DOM tree construction; CSS (P4) is done before full DOM link verification to match the requested order.

## What I Did Last Session
- Replaced `winit` with `eframe` + `egui` in root `Cargo.toml` and `parts/p1_window/Cargo.toml`.
- Migrated `src/main.rs` and `parts/p1_window/src/main.rs` to `eframe::App` / `run_native` API.
- Added black `egui` background via `CentralPanel` + `rect_filled`.
- Updated `agent.md` to state we never used winit and always used eframe/egui.
- Did NOT touch `src/engine/js.rs`, `agent.md` (other than tech stack/note), or other parts.
- Did NOT integrate with fetch, DOM, CSS, layout, or GPU.

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
Start P3 `dom`: create `parts/p3_dom/src/main.rs` with `html5ever` tokenizer wrapper and `Rc<Node>` tree. Implement `first_child()`, `next_sibling()`, `parent_node()`. Verify with `cargo check` standalone. Do not integrate.

## Notes
- Each part must compile independently (`cargo check` in its directory).
- No integration during part development; assembly is a separate phase (ASM1).
- If `cargo check` fails >3 prompts, add to `todo.md` and simplify.
- Human review required before touching `src/main.rs` or `src/engine/js.rs`.
- After fixing `src/main.rs`, run `cargo clean && cargo run` to clear stale build artifacts.
- Window layer is eframe/egui; winit was never used.
