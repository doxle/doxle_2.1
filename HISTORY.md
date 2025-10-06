# doxle_2.1 - Project History

## Project Overview
**Goal:** Build a desktop/mobile app that can load images, pan/zoom, and draw polygons on them.

**Key Design Decision:** Keep core logic architecture-independent so we can build with either:
- **Bevy-in-Dioxus** (Dioxus hosts the window, Bevy renders to a canvas element)
- **Dioxus-in-Bevy** (Bevy hosts the window, Dioxus UI overlaid as a plugin)

This lets us compare both approaches without rewriting the core logic.

---

## Project Structure

```
doxle_2.1/                    (workspace root)
├── Cargo.toml                (workspace config)
├── TODO.md                   (current tasks)
├── HISTORY.md                (this file)
├── core/                     (architecture-independent library)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs            (module exports)
│       ├── geom.rs           (Vec2, Rect - basic types)
│       ├── viewport.rs       (pan/zoom math)
│       ├── polygon.rs        (polygon drawing logic)
│       └── app_state.rs      (orchestration)
├── bevy-app/                 (Dioxus-in-Bevy: Bevy hosts, Dioxus UI plugin)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── dioxus-app/               (Bevy-in-Dioxus: Dioxus hosts, Bevy renders to canvas)
    ├── Cargo.toml
    └── src/
        └── main.rs
```

---

## Core Modules Design

### 1. `geom.rs` - Basic geometry types
- `Vec2` - 2D vector/point (x, y)
- `Rect` - Rectangle (x, y, width, height)
- No dependencies on Bevy or Dioxus

### 2. `viewport.rs` - Pan/Zoom logic
- Manages camera position and zoom level
- Converts between screen coordinates and world coordinates
- Methods: `pan()`, `zoom()`, `screen_to_world()`, `world_to_screen()`
- Pure math - no UI framework

### 3. `polygon.rs` - Polygon drawing
- Stores polygon vertices
- Methods: `add_point()`, `close()`, `contains_point()`, `area()`
- Geometric calculations only

### 4. `app_state.rs` - Orchestration
- Combines viewport + polygon + image state
- Handles user interactions at business logic level
- Returns what changed (for rendering)

---

## Architecture Details (CORRECTED)

### Understanding the Two Approaches

Based on dioxus-examples repository at `/Users/doxle/Desktop/dioxus-examples/examples/`:

#### Architecture 1: Bevy-in-Dioxus (SIMPLER)
**Example:** `/dioxus-examples/examples/bevy/`
- **Who hosts:** Dioxus (line 31: `dioxus_native::launch_cfg(app, ...)`)
- **Window creation:** Dioxus creates and manages the window
- **Bevy's role:** Renders to a `canvas` element (lines 104-106)
- **Integration:** Bevy uses custom paint source (`use_wgpu` hook)
- **UI:** Full Dioxus UI with HTML/CSS (overlay, buttons, inputs)
- **Complexity:** Lower - Dioxus handles most of the plumbing
- **Our package:** `dioxus-app/`

#### Architecture 2: Dioxus-in-Bevy (MORE COMPLEX)
**Example:** `/dioxus-examples/examples/native-headless-in-bevy/`
- **Who hosts:** Bevy (line 21: `App::new()` with Bevy App)
- **Window creation:** Bevy creates and manages the window
- **Dioxus role:** Added as a plugin (line 23: `.add_plugins(DioxusInBevyPlugin ...)`)
- **Integration:** Custom plugin system with channels for communication
- **UI:** Dioxus renders as overlay on Bevy's render pipeline
- **Complexity:** Higher - requires custom plugin and message passing
- **Our package:** `bevy-app/`

### Why This Architecture?

**Problem:** We want to try both Bevy-in-Dioxus and Dioxus-in-Bevy

**Solution:** Keep 80% of code (core logic) separate from UI

**Benefits:**
- Test core logic without any UI framework
- Switch architectures by only changing adapters (10-20% of code)
- All business rules stay in one place

### Key Insight from Examples

**Bevy-in-Dioxus is simpler because:**
1. Dioxus native already has window management
2. Bevy just needs to render to a surface
3. Standard Dioxus event handling works out of the box
4. Less custom glue code needed

**Dioxus-in-Bevy is more powerful because:**
1. Full access to Bevy's ECS and rendering pipeline
2. Better for game-like features or 3D
3. More control over rendering order
4. But requires custom plugin architecture

---

## Current Progress

### Completed:
✅ Created workspace structure (core/, bevy-app/, dioxus-app/)
✅ Planned architecture
✅ Decided on module names (viewport, polygon, app_state)
✅ Implemented minimal core library (geom.rs, polygon.rs, app_state.rs)
✅ Verified correct architecture mapping from dioxus-examples:
   - dioxus-app = Bevy-in-Dioxus (simpler, based on /examples/bevy/)
   - bevy-app = Dioxus-in-Bevy (based on /examples/native-headless-in-bevy/)

### Next Steps:
1. Build dioxus-app (Bevy-in-Dioxus - simpler)
   - Follow /examples/bevy/ pattern
   - Dioxus hosts window
   - Bevy renders to canvas
   - Minimal: click to add polygon points
2. Build bevy-app (Dioxus-in-Bevy)
   - Follow /examples/native-headless-in-bevy/ pattern
   - Bevy hosts window
   - Dioxus UI as plugin
3. Add viewport (pan/zoom) in Step 2

---

## Files to Create (Next Session)

### 1. `/Users/doxle/Desktop/doxle_2.1/Cargo.toml`
```toml
[workspace]
members = [
    "core",
    "bevy-app",
    "dioxus-app",
]
resolver = "2"
```

### 2. `/Users/doxle/Desktop/doxle_2.1/TODO.md`
```markdown
# doxle_2.1 - TODO

## Project Goal
Build an app that can:
- Load an image
- Pan and zoom the image
- Draw polygons on the image
- Works on desktop and mobile

## Architecture
Keep core logic independent of UI framework

## Next Steps
1. [ ] Build core/viewport.rs - Pan/zoom math
2. [ ] Build core/polygon.rs - Drawing logic
3. [ ] Build core/app_state.rs - State management
4. [ ] Add tests
5. [ ] Build bevy-app adapter
6. [ ] Build dioxus-app adapter
```

---

## Important Concepts

### Cargo Workspace
- Manages multiple packages in one project
- One `cargo build` builds everything
- Shared `target/` folder (saves space)
- Use `cargo run -p bevy-app` to run specific package

### `--lib` flag
- `cargo new --lib` creates a library (not executable)
- Has `src/lib.rs` instead of `src/main.rs`
- Used for code other projects can depend on

### Architecture Independence
- Core modules have NO imports from Bevy or Dioxus
- Only use standard Rust types
- Adapters convert between core and UI framework

---

## Learning Approach
User prefers to **type out code manually** with line-by-line explanations (not auto-generated) for better learning.

---

## Session End
Date: 2025-10-01
Status: Ready to start coding core modules
Next: Type out Cargo.toml and TODO.md, then build core library
