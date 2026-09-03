# Focus Browser — Agent Specification

## Project Identity
- **Name:** Focus Browser (codename: `focus-browser`)
- **Type:** Hobby project, 12-week sprint
- **Target Audience:** Students, deep-work users, people who want one tab and zero distractions
- **Philosophy:** One tab. Zero IPC. All RAM and CPU dedicated to the current page. Nothing runs in the background. Single-tab focus lets us optimize that one tab to the extreme: fastest startup, lowest RAM, maximum parallelization. Multi-tab comes later; audio is out; WebAssembly deferred until needed.
- **AI Assistant:** Inkling Small only (1M context, 93 tok/s)
- **Note:** We never used winit. The window layer has always been implemented with eframe and egui.

---

## How We Work (Part-Based Development)

**Do NOT build the whole browser at once.** We build **isolated parts**, verify each part works, then **assemble** them in a final integration phase.

### The Workflow
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   PART A    │────→│   PART B    │────→│   PART C    │
│  (isolated) │     │  (isolated) │     │  (isolated) │
└─────────────┘     └─────────────┘     └─────────────┘
│                   │                   │
└───────────────────┴───────────────────┘
│
┌─────────────┐
│  ASSEMBLE   │
│  (connect   │
│   parts)     │
└─────────────┘
plain

**Rules:**
1. **One part per session.** Never try to implement two parts in one prompt.
2. **Parts compile independently.** Each part has its own `cargo check` target.
3. **No integration until all parts pass.** Do not wire Part A to Part B until both are done.
4. **Assembly is its own phase.** The final "connect everything" session is explicitly scheduled.

---

## Part Definitions

### Phase 1: Foundation (Weeks 1-3)

| Part | Name | What It Does | Success Criteria |
|------|------|--------------|----------------|
| **P1** | `window` | `eframe` + `egui` window + event loop | Window opens, renders black background, handles close/resize |
| **P2** | `fetch` | `reqwest` + `tokio` HTTP client | Can `GET https://example.com` and print bytes to console |
| **P3** | `dom` | `html5ever` → DOM tree | Parses HTML string into traversable DOM (`Rc<Node>`) |
| **P4** | `styles` | `stylo` trait impl + cascade | Implements `TNode`/`TElement`, computes styles for a DOM |
| **P5** | `layout` | `taffy` box tree | Converts styled DOM into positioned rectangles |
| **P6** | `gpu` | `wgpu` + `cosmic-text` | Renders colored rectangles and text to screen |
| **ASM1** | `static-render` | **ASSEMBLY:** P1+P2+P3+P4+P5+P6 | Fetches example.com, parses, styles, layouts, renders |

### Phase 2: Interactivity (Weeks 4-7)

| Part | Name | What It Does | Success Criteria |
|------|------|--------------|----------------|
| **P7** | `js-engine` | `boa_engine` context | Creates JS runtime, exposes `console.log` |
| **P8** | `dom-api` | DOM bridge (`querySelector`, etc.) | JS can read/write DOM nodes via Rust bridge |
| **P9** | `events` | Input → hit test → dispatch | Click on screen coordinate → find element → fire event |
| **P10** | `nav` | URL bar + history | Type URL → fetch → render. Back button works. |
| **ASM2** | `interactive` | **ASSEMBLY:** ASM1+P7+P8+P9+P10 | Can browse Hacker News, click links |

### Phase 3: Polish (Weeks 8-12)

| Part | Name | What It Does | Success Criteria |
|------|------|--------------|----------------|
| **P11** | `scroll` | Scroll events → re-paint | Mouse wheel moves content, maintains 120fps |
| **P12** | `cache` | HTTP disk cache | Revisit page = instant load from disk |
| **P13** | `session` | Save/restore + GPU texture cache | Serialize state, recall from texture blit |
| **P14** | `chrome-ui` | URL bar, buttons, dark mode | Usable browser chrome |
| **ASM3** | `ship` | **ASSEMBLY:** Everything | Working browser, <1s startup, <150MB RAM |

---

## File Structure
focus-browser/
├── .cargo/
│   └── config.toml
├── .github/
│   └── workflows/
│       └── build.yml
├── parts/                   # ISOLATED PARTS (development)
│   ├── p1_window/
│   │   └── src/main.rs
│   ├── p2_fetch/
│   │   └── src/main.rs
│   ├── p3_dom/
│   │   └── src/main.rs
│   ├── p4_styles/
│   │   └── src/main.rs
│   ├── p5_layout/
│   │   └── src/main.rs
│   ├── p6_gpu/
│   │   └── src/main.rs
│   ├── p7_js_engine/
│   │   └── src/main.rs
│   ├── p8_dom_api/
│   │   └── src/main.rs
│   ├── p9_events/
│   │   └── src/main.rs
│   ├── p10_nav/
│   │   └── src/main.rs
│   ├── p11_scroll/
│   │   └── src/main.rs
│   ├── p12_cache/
│   │   └── src/main.rs
│   ├── p13_session/
│   │   └── src/main.rs
│   └── p14_chrome_ui/
│       └── src/main.rs
├── src/                     # FINAL ASSEMBLY (production)
│   ├── main.rs              # HUMAN REVIEW ONLY
│   ├── browser/
│   │   ├── chrome.rs
│   │   ├── session.rs
│   │   └── history.rs
│   ├── engine/
│   │   ├── html.rs
│   │   ├── css.rs
│   │   ├── layout.rs
│   │   ├── render.rs
│   │   ├── text.rs
│   │   └── js.rs            # HUMAN REVIEW ONLY
│   └── net/
│       └── fetch.rs
├── progress.md              # PROGRESS TRACKING
├── todo.md                  # ISSUE/TASK TRACKING
├── Cargo.toml               # Workspace root
├── agent.md                 # THIS FILE
└── README.md
plain

**Each `parts/pN_*/` has its own `Cargo.toml`.** It depends only on the crates needed for that part. It compiles as a standalone binary. This keeps `cargo check` fast and failures isolated.

---

## progress.md (Tracking File)

**Location:** `progress.md` at repo root.

**Updated by:** AI at the end of every session. Human reviews.

**Format:**
```markdown
# Focus Browser Progress

## Current Phase
Phase 1: Foundation

## Completed Parts
- [x] P1 `window` — Window opens, handles events. Done 2026-09-02.
- [x] P2 `fetch` — Can fetch example.com. Done 2026-09-03.
- [ ] P3 `dom` — IN PROGRESS

## Current Part
P3 `dom` — html5ever DOM tree

## What I Did Last Session
Implemented html5ever tokenizer wrapper. DOM tree builds but child nodes not linking correctly.

## Issues / Blockers
- `Rc<Node>` parent/child links create borrow checker issues with mutable refs
- Need to decide: `Rc<RefCell<Node>>` vs `Arena<Node>` vs custom tree

## Next Session Goal
Fix Node tree ownership. Implement `first_child()`, `next_sibling()`, `parent_node()`.

## Notes
- html5ever uses `Tendril` for strings — may need to convert to `String` for our DOM
- Consider using `indextree` crate for arena-based tree instead of Rc
AI Rule: At the end of every session, append to progress.md. Do not overwrite previous entries.
todo.md (Issue Tracking)
Location: todo.md at repo root.
Updated by: AI when issues are found or resolved.
Format:
Markdown
Copy
Code
Preview
# Focus Browser Todo / Issues

## Active
- [ ] P3: Fix `Rc<RefCell<Node>>` borrow panic in `append_child`
- [ ] P4: Stylo `TNode` trait requires `NodeData` type — need to define our enum
- [ ] P6: `cosmic-text` font atlas texture format mismatch with wgpu

## Blocked (Waiting on other parts)
- [ ] ASM1: Cannot assemble until P6 is done
- [ ] P8: Waiting on P7 (js-engine) to be complete

## Resolved
- [x] P1: winit window not closing on X button → fixed by handling `WindowEvent::CloseRequested`
- [x] P2: reqwest blocking in async context → fixed by using `tokio::main`

## Backlog (Not in current phase)
- [ ] P11: Scroll inertia physics
- [ ] P13: GPU texture LRU eviction policy
- [ ] Dark mode shader
AI Rule: When you encounter a problem you can't solve in the current session, add it to todo.md under Active or Blocked. When you return, check todo.md first.
Session Prompt Template
Paste this at the start of every session:
plain
[PROJECT CONTEXT]
I am building Focus Browser, a single-tab Rust browser. Read agent.md for full spec.

[CURRENT STATE]
progress.md:
[paste contents of progress.md]

todo.md:
[paste contents of todo.md]

[THIS SESSION]
I want to work on: [Part Name, e.g., P3 `dom`]

[PREVIOUS CODE]
[paste the current code for this part]

[PROBLEM / GOAL]
[Describe what to do this session]

[CONSTRAINTS]
- This part must compile independently with `cargo check`
- Do not touch other parts
- Do not integrate with other parts yet
- If stuck >3 prompts, add to todo.md and suggest a simpler approach
Assembly Rules (Critical)
Assembly sessions are EXPLICITLY scheduled. Do not "just connect things" at the end of a part session.
When to Assemble
After ALL parts in a phase are marked [x] in progress.md
In a dedicated session titled "ASM1: Static Render Assembly"
How to Assemble
Copy working part code into src/ (not move — keep parts/ as reference)
Write glue code in src/main.rs that wires parts together
Delete part-specific main.rs stubs (each part had its own main for testing)
cargo check the full project — fix integration issues only
Do NOT refactor part internals during assembly. If a part needs changes, go back to its part directory, fix it, verify it still works standalone, then re-copy.
Assembly Checklist
Markdown
Copy
Code
Preview
## ASM1 Checklist
- [ ] P1 code copied to `src/browser/window.rs`
- [ ] P2 code copied to `src/net/fetch.rs`
- [ ] P3 code copied to `src/engine/html.rs`
- [ ] P4 code copied to `src/engine/css.rs`
- [ ] P5 code copied to `src/engine/layout.rs`
- [ ] P6 code copied to `src/engine/render.rs` + `src/engine/text.rs`
- [ ] `src/main.rs` wires all modules
- [ ] `cargo check` passes on full project
- [ ] Binary runs and renders example.com
Constraints (Non-Negotiable)
Table
Constraint	Value	Why
Language	Rust only	No C++ bindings. No unsafe FFI if possible.
Timeline	12 weeks	Ship a working browser by week 12.
Architecture	Single tab, single process, multi-threaded	No process-per-tab. No IPC.
Media	No audio, no video, no WebRTC	Text-only browser.
Hardware	Modern only (≤1 year old)	Vulkan 1.2+, NVMe, 16GB+ RAM. No legacy GPU.
Platform	Linux aarch64 primary	ARM Chromebook/Crostini.
Build	Local cargo check -j1, GitHub Actions release	2.7GB RAM local, 7GB RAM remote compile.
AI	Inkling Small only	1M context, 93 tok/s. No multi-agent overhead.
Technology Stack
Table
Layer	Crate	Purpose
Windowing	eframe + egui	Window + input events + black background rendering
HTTP	reqwest + tokio	Async fetch
HTML	html5ever	Streaming parser → DOM
CSS	stylo (style crate)	Parallel selector matching (Rayon)
Layout	taffy	Flexbox + Grid
JS	boa_engine	Pure Rust ECMAScript
GPU	wgpu (Vulkan only)	Direct Vulkan rendering
Text	cosmic-text	GPU text shaping
Images	image	JPEG/PNG/WebP decode
Fallback Rules
Table
If This Happens	Do This
Stylo traits take >3 days	Fall back to lightningcss for this part. Revisit Stylo later.
cargo check fails >3 prompts	Add issue to todo.md, simplify the part, or skip to next part.
Part depends on unfinished part	Mark as Blocked in todo.md. Work on something else.
Inkling Small rate-limited	Wait 5 minutes. Do not switch models.
Performance Targets
Table
Metric	Target
Cold startup	<1s
RAM usage	<150MB
First paint (cached)	<100ms
Scroll	120fps locked
Page recall (texture cache)	<16ms
What We Do NOT Build
No multi-tab / multi-window
No audio / video / WebRTC / media codecs / DRM
No extension API
No adblock (v1)
No WebAssembly (v1)
No legacy GPU support (OpenGL, D3D11, software fallback)
No process sandboxing
Rules for AI
One part per session. Never implement two parts at once.
Each part must cargo check independently before moving on.
Never touch src/main.rs or src/engine/js.rs without explicit human review.
Update progress.md and todo.md at the end of every session.
No integration during part development. Assembly is a separate phase.
No unsafe unless absolutely necessary.
If stuck, simplify. A working simple part is better than a broken complex part.
Test incrementally. example.com before Wikipedia. Wikipedia before Hacker News.
