# Feature: Polygon Wall Drawing System

**Branch:** `2-polygon-wall`  
**Status:** In Development  
**Started:** 2025-10-06

---

## Overview

Building a 2D/3D polygon drawing system for architectural floor plans and wall visualization. Users draw polygon footprints in 2D mode and can switch to 3D mode to view extruded walls.

---

## Core Concepts

### Data Flow
```
User Click → Point(x, y, z) → Polygon → AppState → Bevy Renderer
```

### View Modes
- **2D Mode**: Orthographic top-down view, z=0 plane, drawing floor plans
- **3D Mode**: Perspective angled view, walls extruded to z=3.0m, interior visible

### Key Features
1. Click to place points (red circles, 3px radius)
2. Auto-connect lines between consecutive points
3. Auto-snap to first point (10px threshold) to close polygon
4. Smooth animated transition between 2D and 3D views
5. Hover preview (circle + line from last point)
6. Pop animation on point placement

---

## Architecture Decisions

### ✅ Unified Rendering: Bevy for Both 2D and 3D

**Why Bevy for both:**
- Single rendering pipeline (no coordinate translation)
- Seamless mode switching (just camera transformation)
- Can animate the transition (camera moves smoothly)
- Consistent event handling
- Pan/zoom/rotate works in both modes
- Future-proof for hybrid views

**Rejected:** Canvas 2D + Bevy 3D (would require dual systems)

### ✅ Coordinate System

**World Space (Meters):**
- X-axis: Width (left/right)
- Y-axis: Depth (forward/back)  
- Z-axis: Height (up/down)

**2D Mode:**
- Camera: Orthographic, looking straight down
- All points at z=0

**3D Mode:**
- Camera: Perspective, angled ~45° from above
- Floor polygon at z=0
- Wall tops at z=3.0m (standard ceiling height)

**Origin:** Center of viewport (0, 0, 0)

### ✅ Point Structure

```rust
pub struct Point {
    pub x: f32,  // meters, horizontal
    pub y: f32,  // meters, depth
    pub z: f32,  // meters, height (0 in 2D, varies in 3D)
}
```

**Units:** Meters (Australian standard)  
**Default Wall Height:** 3.0m

---

## UI Design

### Window Configuration

**Minimum Size:** 1280 x 800px (enforced)  
**Resizable:** Yes  
**Maximum:** Full screen (maximize button)  
**Full Screen Mode:** Standard maximize only (not native macOS full screen)

**Canvas Scaling:** When maximized, show more drawing area (not just zoom)

### Navbar Layout

```
┌─────────────────────────────────────────────────────────────┐
│ ●●●  🐕            [2D] [3D]                      (S) ▾     │  60px
├─────────────────────────────────────────────────────────────┤
│                                                             │
│                     Bevy Canvas Area                        │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**Layout:**
- **Left:** macOS traffic lights + Dog logo (32px, no text)
- **Center:** 2D/3D toggle icons (absolutely centered)
- **Right:** User avatar circle with dropdown chevron

**Height:** 60px  
**Background:** `var(--bg-primary)` (theme-aware)  
**Border:** 1px bottom border, subtle divider

### Center: 2D/3D Toggle

**Icon Design:**
- Size: 24px icon, 40x40px clickable area
- Spacing: 8px gap between icons
- Active state: Blue background (#3b82f6), 8px border-radius
- Inactive state: Transparent, gray on hover
- Transition: 150ms smooth

**Behavior:**
- Click to switch modes
- Smooth animated camera transition (0.8-1.0s)
- Ease-in-out animation curve

### Right: User Avatar + Dropdown

**Avatar Circle:**
- Size: 32px diameter
- Background: Yellow (#fbbf24)
- Text: "S" (hardcoded), 14px, centered, white
- Chevron: 12px, 4px gap, rotates 180° when open

**Dropdown Menu (Minimal MVP):**
```
┌──────────────┐
│  Settings    │
│  Theme       │
│  ─────────── │
│  About       │
└──────────────┘
```

**Dropdown Styling:**
- Card-style with shadow
- Border-radius: 8px
- Padding: 8px vertical, 0px horizontal
- Item padding: 12px vertical, 16px horizontal
- Hover: Subtle background (#1a1a1a in dark)
- Animation: Fade + slide down (150ms)

**Position:**
- Aligned to right edge of navbar
- 8px gap from top
- z-index: 1000
- Click outside to dismiss

---

## Theme System

### Implementation: Hybrid Approach

**Rust Enum + CSS Variables:**
```rust
enum Theme { 
    Dark,  // Default
    Light 
}
```

**CSS Variables:**
```css
/* Dark Mode (Default) */
--bg-primary: #000000;
--bg-secondary: #1a1a1a;
--text-primary: #ffffff;
--text-secondary: #a0a0a0;
--accent-blue: #3b82f6;
--accent-yellow: #fbbf24;
--line-color: #ef4444;  /* red for polygon lines */
--border-color: #333333;

/* Light Mode */
--bg-primary: #ffffff;
--bg-secondary: #f5f5f5;
--text-primary: #1a1a1a;
--text-secondary: #666666;
/* accents stay similar, adjust if needed */
```

**Auto-Detection:**
- On app launch, detect system theme preference
- Apply matching theme
- User can override in dropdown (saved to app storage)
- Optional: Listen to system theme changes in real-time

---

## Typography

**Primary Font:** SF Pro (macOS system font)  
**Fallback Chain:** `-apple-system, BlinkMacSystemFont, "SF Pro", "Helvetica Neue", sans-serif`

**Font Sizes:**
- Navbar items: 14px, medium (500)
- Dropdown items: 13-14px, regular (400)
- Canvas measurements: 12-13px, regular
- Labels/tooltips: 11px, medium (500)

**Rationale:** SF Pro is native to macOS, zero bundle size, optimized for UI, matches Warp's aesthetic

---

## Drawing Interaction

### Click Behavior

1. **Mouse down on canvas** → Capture x, y coordinates
2. **Ray cast to z=0 plane** → Convert to world space (meters)
3. **Check snap distance** → If within 10px of first point, snap to close polygon
4. **Add point** → `app_state.handle_click(Point::new(x, y, 0.0))`
5. **Render** → Draw red circle (3px), connect line to previous point
6. **Animation** → Brief "pop" on circle appearance

### Hover Preview

**Before clicking:**
- Show preview circle at mouse position (lighter red/pink)
- If polygon has points, show preview line from last point to cursor
- If near first point (within 10px), highlight first point (pulse or scale)

**Cursor changes:**
- Default: Crosshair (drawing mode)
- Near first point: "Close" cursor or highlight

### Polygon Closing

**Auto-snap behavior:**
- When mouse is within 10px of first point
- First point highlights (scale 1.2x or pulse)
- Preview line snaps to first point
- On click: Polygon closes automatically
- Closing line rendered
- Brief animation: Lines pulse once, color shift to "completed"

**After closing:**
- Start new polygon on next click (support multiple polygons)
- Option to lock/edit later (future feature)

---

## 2D ↔ 3D Transition Animation

### 2D → 3D Sequence

**Camera Animation (0.8-1.0s):**
1. Pull back (increase z position)
2. Tilt down ~45° (rotate to see walls)
3. Switch projection: Orthographic → Perspective
4. Ease-in-out timing

**Geometry Animation (simultaneous):**
1. Polygon points extrude from z=0 → z=3.0
2. Walls "grow" upward
3. Fade-in shadows and depth cues

### 3D → 2D Sequence

**Camera Animation:**
1. Move overhead (back to top-down)
2. Rotate to straight down (0° tilt)
3. Switch projection: Perspective → Orthographic

**Geometry Animation:**
1. Walls collapse to z=0
2. Simplify to flat lines
3. Fade-out 3D-only elements

**User Experience:**
- Animation is automatic on mode toggle
- Smooth, not jarring
- Can be interrupted (click again mid-animation reverses)

---

## Rendering Details

### 2D Mode Rendering

**Elements:**
- Red circles at each point (3px radius, filled)
- Red lines connecting consecutive points
- Closing line (last → first) when polygon is closed
- Preview circle and line (lighter shade)

**Drawing Order:**
1. Grid (background, optional)
2. Completed polygons (filled or wireframe)
3. Active polygon lines
4. Active polygon points (circles)
5. Preview elements (on top)

### 3D Mode Rendering

**Geometry:**
- Floor polygon (z=0, could be filled or outline)
- Vertical walls (quads from each edge, z=0 to z=3.0)
- Optional: Roof/cap at z=3.0

**Materials:**
- Walls: Semi-transparent or solid
- Edges: Highlighted lines
- Floor: Grid or solid color

**Camera:**
- Position: Above and to side of polygon center
- Target: Center of polygon
- FOV: 60-70°

**Lighting:**
- Ambient light (see all walls clearly)
- Optional: Directional light for shadows

---

## Visual Design Tokens

### Colors (Dark Mode)

```
Background:   #000000
Surface:      #1a1a1a
Border:       #333333
Text Primary: #ffffff
Text Secondary: #a0a0a0

Accent Blue:  #3b82f6  (active state)
Accent Yellow: #fbbf24 (user avatar)
Line Color:   #ef4444  (polygon lines, red)
Preview:      #fca5a5  (lighter red)
Success:      #10b981  (future: completed state)
```

### Spacing Scale

```
xs:  4px
sm:  8px
md:  12px
lg:  16px
xl:  20px
2xl: 24px
3xl: 32px
```

### Border Radius

```
small:  4px  (buttons)
medium: 8px  (cards, active states)
large:  12px (modals)
full:   50%  (avatar circle)
```

### Shadows

```
sm: 0 1px 2px rgba(0,0,0,0.05)
md: 0 4px 6px rgba(0,0,0,0.1)
lg: 0 10px 15px rgba(0,0,0,0.15)
```

---

## Implementation Phases

### Phase 1: Foundation & 2D Drawing ✅ NEXT
- [ ] Black window background
- [ ] Single unified navbar (60px)
  - [ ] Left: Traffic lights + Dog logo placeholder (32px)
  - [ ] Center: 2D/3D toggle buttons (text placeholders)
  - [ ] Right: User avatar "S" with chevron
- [ ] User dropdown menu (Settings, Theme, About)
- [ ] Theme system (CSS variables, dark mode default)
- [ ] Window config (1280x800 min, resizable)
- [ ] Bevy canvas setup (2D orthographic camera)
- [ ] Click event handling (get world coordinates)
- [ ] Add points to polygon state
- [ ] Render red circles at points (3px)
- [ ] Render lines connecting points
- [ ] Hover preview (circle + line)
- [ ] Auto-snap polygon closing (10px threshold)

### Phase 2: 3D Static View
- [ ] Update Point struct (add z: f32)
- [ ] Create 3D wall geometry from closed polygon
- [ ] 3D mode camera setup (perspective, angled)
- [ ] Toggle between 2D/3D (instant switch, no animation)
- [ ] Render walls as vertical quads

### Phase 3: Smooth Transition Animation
- [ ] Animate camera: 2D → 3D transformation
- [ ] Animate camera: 3D → 2D transformation
- [ ] Animate wall extrusion (geometry grows)
- [ ] Timing curve (ease-in-out, 0.8-1.0s)
- [ ] Interruptible animation (can reverse mid-transition)

### Phase 4: Camera Controls
- [ ] 3D mode: Orbit/rotate around polygon
- [ ] 3D mode: Zoom (mouse wheel)
- [ ] 3D mode: Pan (shift+drag or middle mouse)
- [ ] 2D mode: Pan (drag)
- [ ] 2D mode: Zoom (mouse wheel)
- [ ] Touch support (future: pinch, two-finger)

### Phase 5: Polish & Assets
- [ ] Replace placeholder dog logo with actual SVG
- [ ] Replace 2D/3D text with actual SVG icons
- [ ] Add pop animation on point placement
- [ ] Refine hover states and transitions
- [ ] Add measurements display (show dimensions on lines)
- [ ] Add grid toggle (background grid)
- [ ] Add undo last point button
- [ ] Add clear all button
- [ ] Light mode theme implementation
- [ ] Theme toggle in dropdown functional
- [ ] About dialog content

---

## Assets Needed (To Be Provided)

1. **Dog Logo SVG** (32x32px recommended size)
2. **2D Icon SVG** (24x24px, floor plan representation)
3. **3D Icon SVG** (24x24px, isometric cube/walls)
4. **Optional: Helvetica Neue font** (if preferred over SF Pro)

---

## Technical Notes

### Bevy Integration

**Existing Setup:**
- `demo_renderer.rs` - Current spinning cube example
- `bevy_renderer.rs` - Bevy/Dioxus bridge
- WGPU limits configured for complex scenes

**New Requirements:**
- 2D line rendering (can use `bevy_prototype_lyon` or custom mesh)
- 3D mesh generation from polygon points
- Camera controller component
- Click ray casting for world space coordinates

### State Management

**Current:**
- `AppState` in `doxle-core`
- `Polygon` and `Point` structs

**Additions:**
- `ViewMode` enum (2D/3D)
- Camera state (position, target, rotation)
- Theme state (Dark/Light)
- Dropdown open/closed state
- Preview point state (hover position)

### Event Handling

**Mouse Events:**
- `onclick` - Add point
- `onmousemove` - Update preview
- `onmouseenter/leave` - Show/hide preview
- `onwheel` - Zoom (when implemented)

**Keyboard Shortcuts (Future):**
- `Esc` - Cancel current polygon
- `Z` - Undo last point
- `Space` - Switch 2D/3D
- `Delete` - Clear polygon

---

## Testing Checklist

### UI Tests
- [ ] Navbar renders correctly at min width (1280px)
- [ ] Navbar renders correctly at max width (full screen)
- [ ] 2D/3D toggle visual states
- [ ] User dropdown opens/closes
- [ ] Theme switch updates all colors
- [ ] Dark mode default on launch
- [ ] Light mode toggle works

### Drawing Tests
- [ ] Click adds point at correct world coordinates
- [ ] Lines connect consecutive points
- [ ] Hover preview shows correctly
- [ ] Auto-snap detects first point within 10px
- [ ] Polygon closes on snap click
- [ ] New polygon starts after closing
- [ ] Multiple polygons supported

### 3D Tests
- [ ] 3D mode shows walls extruded to 3.0m
- [ ] Camera angle shows interior
- [ ] Transition animation smooth (no jank)
- [ ] Can interrupt animation
- [ ] Coordinates consistent between modes

### Performance Tests
- [ ] Smooth at 60fps with 10 polygons
- [ ] Smooth at 60fps with 100 total points
- [ ] Animation doesn't drop frames
- [ ] Resize window doesn't lag

---

## Known Limitations & Future Work

### Current Scope (MVP)
- Single floor level (z=0 to z=3.0)
- Basic wall visualization (no doors/windows)
- No measurement tools yet
- No export functionality
- Desktop only (no web/mobile)

### Future Enhancements
- Multi-story buildings (multiple z-levels)
- Door and window placement
- Wall thickness (currently zero-width lines)
- Furniture/object placement
- Measurement annotations
- Export to common formats (DXF, IFC, PDF)
- Collaboration features
- Undo/redo stack
- Selection and editing of existing polygons
- Copy/paste polygons
- Snap to grid
- Angle constraints (orthogonal mode)

---

## Questions & Decisions Log

**2025-10-06:**
- ✅ Use Bevy for both 2D and 3D (unified rendering)
- ✅ Point struct includes z-coordinate
- ✅ Auto-snap polygon closing at 10px threshold
- ✅ Meters as primary unit (Australian standard)
- ✅ Wall height default: 3.0m
- ✅ Dropdown menu: Minimal (Settings, Theme, About)
- ✅ Full screen: Standard maximize only
- ✅ Canvas scaling: Show more area when maximized
- ✅ Dropdown animation: Fade + slide (150ms)
- ✅ Logo size: 32px
- ✅ SF Pro as primary font
- ✅ Theme auto-detection on launch
- ✅ Single unified navbar (not two-tier)

---

## Development Commands

```bash
# Run the app
cargo run --package dioxus-app

# Run with tracing
cargo run --package dioxus-app --features tracing

# Build release
cargo build --release --package dioxus-app

# Run tests
cargo test --package doxle-core

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy
```

---

## Context Recovery

If you lose context in future sessions, read this document first. Key things to remember:

1. **We're building a 2D/3D polygon drawing tool** - draw floor plans, view as walls
2. **Everything in Bevy** - unified rendering, no separate 2D system
3. **Coordinate system is meters** - architectural focus, Australian standard
4. **Point has x,y,z** - z=0 for 2D floor, z=3.0 for wall tops
5. **Warp-inspired UI** - clean, simple, single navbar with dropdown
6. **Theme system ready** - CSS variables, auto-detect OS preference
7. **Phase 1 is next** - build navbar, 2D drawing, basic interaction

**Current Branch:** `2-polygon-wall`  
**Previous Branch:** `1-spinning-cube` (reference for Bevy integration)  
**Main Branch:** Clean slate, just initial setup

---

*Last Updated: 2025-10-06*
