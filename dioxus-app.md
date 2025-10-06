# dioxus-app - Bevy-in-Dioxus Architecture

## Overview
This app follows the **Bevy-in-Dioxus** pattern where:
- **Dioxus** hosts the window and manages UI
- **Bevy** renders graphics into a Dioxus canvas element

## Reference Example
Based on: `/Users/doxle/Desktop/dioxus-examples/examples/bevy/`

## What is dioxus-native?

`dioxus-native` is Dioxus's desktop renderer **built on Blitz**.

**Blitz** is a native HTML/CSS rendering engine written in Rust:
- Renders HTML, CSS, SVG natively (no browser/webview)
- Built on WGPU (same graphics API as Bevy!)
- Think of it as "a browser engine for native apps"
- Source: `/dioxus-examples/packages/native/Cargo.toml` (line 6: "Native renderer for Dioxus based on blitz")

### The Full Stack

```
Your Dioxus App (RSX/HTML-like syntax)
    ↓
dioxus-native (uses Blitz renderer)
    ↓
Blitz (HTML/CSS rendering engine)
    ↓
WGPU (graphics API)
    ↓
GPU
```

### In Our Bevy-in-Dioxus App

```
┌────────────────────────────────────┐
│ Dioxus-native (Blitz renders):     │
│  - Window creation                 │
│  - UI elements (buttons, text)     │
│  - Canvas element (container)      │
│            ↓                       │
│  ┌─────────────────────────────┐  │
│  │ Bevy renders into canvas:   │  │
│  │  - Polygon points           │  │
│  │  - Lines                    │  │
│  └─────────────────────────────┘  │
│                                    │
│ Both use WGPU! ✅                  │
└────────────────────────────────────┘
```

**Key insight:** Blitz and Bevy both use WGPU underneath, which is why they integrate smoothly.

---

## Key Concepts

### Who Controls What?

| Component | Owner | Responsibility |
|-----------|-------|----------------|
| Window creation | Dioxus | Creates and manages the native window |
| UI layout | Dioxus | Buttons, text, canvas positioning |
| Canvas element | Dioxus | Provides `<canvas>` element as container |
| Graphics rendering | Bevy | Draws into the canvas using WGPU |
| Event capture | Dioxus | Captures clicks, passes to core logic |
| State management | Core | Our `AppState` (architecture-independent) |

---

## How Canvas Works (The Confusing Part!)

### Common Confusion ❌
- "Canvas belongs to Bevy" ❌
- "Dioxus targets Bevy canvas" ❌

### Reality ✅
- **Dioxus provides the `<canvas>` element** (the container/frame)
- **Bevy renders INTO that canvas** (the graphics/painting)

### Analogy
Think of a picture frame:
- **Dioxus = The picture frame** (provides space, position, size)
- **Bevy = The painting** (fills the space with graphics)

---

## Code Flow (from example)

### 1. Main Entry Point
**File:** `main.rs` (line 26-32)
```rust
fn main() {
    let config: Vec<Box<dyn Any>> = vec![Box::new(limits())];
    dioxus_native::launch_cfg(app, Vec::new(), config);
    // ↑ Dioxus creates window and runs the app
}
```

### 2. App Component (UI Layout)
**File:** `main.rs` (line 34-75)
```rust
fn app() -> Element {
    rsx!(
        div { id:"overlay",
            h2 { "Control Panel" },
            button { /* ... */ }  // ← Dioxus UI elements
        }
        SpinningCube { color }    // ← Component with Bevy canvas
    )
}
```

### 3. Canvas Component with Bevy
**File:** `main.rs` (line 92-110)
```rust
fn SpinningCube(color: Memo<Color>) -> Element {
    // Create Bevy rendering context
    let paint_source = DemoPaintSource::new();
    let paint_source_id = use_wgpu(move || paint_source);
    // ↑ This creates the Bevy renderer
    
    rsx!(
        canvas {
            id: "demo-canvas",
            "src": paint_source_id  // ← Connect Bevy to this canvas
        }
        // ↑ Dioxus canvas element (the container)
        // Bevy will render inside this element
    )
}
```

---

## The Connection: `use_wgpu` Hook

**What it does:**
1. Creates a Bevy rendering pipeline (using WGPU graphics API)
2. Returns a `paint_source_id` (handle to the renderer)
3. Dioxus canvas uses this ID to display what Bevy renders

**Think of it like:**
```
┌─────────────────────────────────────┐
│ Dioxus Window                       │
│  ┌───────────────────────────────┐  │
│  │ <canvas src="bevy_renderer">  │  │
│  │                               │  │
│  │   [Bevy draws here]           │  │
│  │                               │  │
│  └───────────────────────────────┘  │
│  <button>Clear</button>             │
└─────────────────────────────────────┘
```

---

## Our Implementation Plan

### File Structure
```
dioxus-app/
├── Cargo.toml
└── src/
    ├── main.rs       (entry point, launch Dioxus)
    ├── ui.rs         (Dioxus UI components)
    └── renderer.rs   (Bevy rendering logic)
```

### What We'll Build

#### 1. `main.rs` - Entry Point
```rust
fn main() {
    // Launch Dioxus
    dioxus_native::launch(app);
}
```

#### 2. `ui.rs` - Dioxus UI
```rust
fn app() -> Element {
    let mut app_state = use_signal(|| AppState::new());  // From core!
    
    rsx!(
        div {
            h1 { "Polygon Drawing App" }
            
            // The canvas where Bevy draws
            PolygonCanvas { 
                app_state: app_state.read().clone() 
            }
            
            // Controls
            button {
                onclick: move |_| app_state.write().clear_polygon(),
                "Clear"
            }
            
            p { "Points: {app_state.read().polygon.point_count()}" }
        }
    )
}
```

#### 3. `renderer.rs` - Bevy Rendering
```rust
// Custom paint source that reads from AppState
// Draws:
// - Circles at each polygon point
// - Lines connecting the points

pub struct PolygonRenderer {
    // Bevy rendering state
}

impl PolygonRenderer {
    pub fn draw(&mut self, polygon: &Polygon) {
        // 1. Clear the canvas
        // 2. Draw circles at each point
        // 3. Draw lines connecting points
    }
}
```

---

## Data Flow

```
User clicks on canvas
    ↓
Dioxus captures click event (x, y)
    ↓
Convert to canvas coordinates
    ↓
app_state.handle_click(Vec2::new(x, y))  ← Our core!
    ↓
Core adds point to polygon
    ↓
Dioxus re-renders
    ↓
Bevy renderer reads new polygon state
    ↓
Bevy draws updated graphics
    ↓
User sees new point on canvas
```

---

## Key Differences from Example

| Example (spinning cube) | Our App (polygon) |
|-------------------------|-------------------|
| 3D rotating cube | 2D points and lines |
| Color picker | Clear button |
| Continuous animation | Static until click |
| Demo rendering | Practical drawing tool |

---

## Step-by-Step Build Plan

### Phase 1: Minimal Working App
1. ✅ Set up Cargo.toml dependencies
2. ✅ Create basic Dioxus window
3. ✅ Add canvas element
4. ✅ Connect Bevy renderer to canvas
5. ✅ Handle click events
6. ✅ Draw single point on click

### Phase 2: Polygon Drawing
1. Store multiple points in AppState
2. Draw all points as circles
3. Draw lines connecting points
4. Add clear button

### Phase 3: Polish
1. Show point count
2. Style the UI
3. Test on different screen sizes

---

## Dependencies (Cargo.toml)

```toml
[dependencies]
dioxus = { version = "0.7", features = ["desktop"] }
dioxus-native = "0.7"
doxle_core = { path = "../core" }  # Our core library!

# For Bevy rendering
wgpu = "0.20"
# ... (will add exact versions when coding)
```

---

## Common Pitfalls to Avoid

1. **Don't put core logic in UI code**
   - ❌ `app_state.polygon.points.push(point)` in ui.rs
   - ✅ `app_state.handle_click(point)` (use core methods)

2. **Don't mix coordinate systems**
   - Screen coords ≠ Canvas coords
   - Always convert before passing to core

3. **Don't make renderer stateful**
   - Renderer should read from AppState
   - Don't store polygon in renderer

---

## Questions to Answer Before Coding

### Visual Style
- Point size: 5px radius circles?
- Line thickness: 2px?
- Colors: Red points (#FF0000), Blue lines (#0000FF)?

### Interaction
- Show point numbers? (0, 1, 2, ...)
- Close polygon automatically? (connect last to first)
- Or keep as open polyline?

### UI Layout
- Canvas size: Fill window? Fixed size?
- Controls: Top, bottom, or side panel?
- Show last clicked coordinate?

---

## Next Steps

Once we answer the questions above, we'll:
1. Write minimal `Cargo.toml`
2. Implement `main.rs` (5-10 lines)
3. Implement `ui.rs` (basic canvas + button)
4. Implement `renderer.rs` (draw circles + lines)
5. Test and iterate

---

**Date Created:** 2025-10-03
**Status:** Planning phase - ready to code once design decisions made
