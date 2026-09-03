# Focus Browser Todo / Issues

## Active / Doing
- [x] P1 `window`: `eframe` + `egui` window opens, black background, handles close/resize. Verified standalone in `parts/p1_window/`. We never used winit.
- [x] P2 `fetch`: `reqwest` + `tokio` HTTP client. Verified standalone in `parts/p2_fetch/`. Integrated directly into `src/main.rs` via `tokio::runtime::Runtime`.
- [x] Build: `openssl-sys` failure fixed — `reqwest` now `default-features = false, features = ["rustls-tls"]`. No system OpenSSL needed.
- [x] GPU-first: `src/main.rs` selects `IntegratedGpu` adapter (`Virtio-GPU`) and suppresses Vulkan validation noise (`WGPU_VALIDATION=0`).
- [ ] Verify skeleton `cargo run`: confirm window + fetch + GPU adapter selection work together in root binary. Plan: run now.
- [ ] P3 `dom`: Implement `html5ever` → `Rc<Node>` tree. Need to decide `Rc<RefCell<Node>>` vs arena (`indextree`). Must implement `first_child()`, `next_sibling()`, `parent_node()`.
- [ ] Multi-threaded optimization (later): Add `rayon` to P3 parser / P5 layout for CPU parallelism; keep GPU parts on `wgpu` thread.

## Blocked (Waiting on other parts)
- [ ] ASM1 `static-render`: Cannot assemble until P1, P2, P3, P4, P5, P6 all pass independently.
- [ ] P8 `dom-api`: Waiting on P7 (`js-engine`) and P3 (`dom`).
- [ ] P9 `events`: Waiting on P3 (`dom`) for hit-test targets.
- [ ] P10 `nav`: Waiting on P2 (`fetch`) and P3 (`dom`).

## Resolved
- [x] P1: Window not closing on X button → fixed by `eframe` viewport close handling.
- [x] P1: Resize events not logged → handled by `egui` viewport.
- [x] P1: Black screen requested → implemented via `egui::CentralPanel` + `rect_filled`.
- [x] P2: `reqwest` blocking in async context → fixed by using `tokio::main` in standalone; integrated via `Runtime::new()` in skeleton.
- [x] P2: Fetch integrated into skeleton — `src/main.rs` now calls `reqwest::get` before `run_native`.
- [x] Build: `openssl-sys` / `pkg-config` error → fixed by disabling `reqwest` default features and using `rustls-tls`.
- [x] GPU selection: `IntegratedGpu` adapter explicitly selected; validation layer suppressed.

## Security / Vulnerability Notes (Critical — Browser Project)
- [ ] P3: HTML parser must sanitize input; do not expose raw `innerHTML` to JS until P8 bridge is audited.
- [ ] P2: `reqwest` must disable redirects to `file://` and restrict to `http/https`; validate URL before fetch.
- [ ] P7: `boa_engine` JS context must not expose `eval` or `Function` constructor until sandbox is defined.
- [ ] P6: `wgpu` shader compilation must not allow arbitrary SPIR-V injection.
- [ ] General: No `unsafe` blocks unless required by `eframe`/`egui`/`wgpu` bindings; prefer safe Rust.
- [ ] General: Single process, no IPC, no background threads that could leak data between tabs (only one tab exists).
- [ ] General: No multi-tab / multi-window; no audio/video/WebRTC; no extension API; no adblock (v1).
- [ ] Note: Window layer uses eframe/egui; winit was never used.

## Backlog (Not in current phase)
- [ ] P11: Scroll inertia physics
- [ ] P13: GPU texture LRU eviction policy
- [ ] Dark mode shader
- [ ] P14: Chrome UI (URL bar, buttons, dark mode)

## Sequence Reminder (User + agent.md)
Window (P1, eframe/egui) → HTML (P3) → CSS (P4) → DOM tree verification (P3) → Fetch (P2) → Layout (P5) → GPU (P6) → Assembly (ASM1).
Do not skip steps. Do not integrate early. Verify each part with `cargo check` before moving on.
Plan: verify skeleton now; then P3.
