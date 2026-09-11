---
name: devlens-ui
description: >-
  Design system, shadcn/ui component mapping, command palette specification,
  keyboard shortcuts, and accessibility guidelines for the DevLens extension.
---

# DevLens UI & Design System

This document outlines the comprehensive UI/UX specifications, design tokens, component mapping, and interaction guidelines for the DevLens browser extension.

## 1. UX Philosophy

DevLens UX = SuperDev Pro's Architecture + Picker.Design's Focus + LoupeKit's Intelligence + toast.log's Invisibility

*   **SuperDev's Architecture:** Shadow DOM sidebar HUD with a pervasive command palette (⌘K) to ensure quick, keyboard-first navigation and a clean separation from host page styles.
*   **Picker's Focus:** One module, one focused screen. No clutter. Features are strictly isolated so developers see exactly what they need for the current task and nothing else.
*   **LoupeKit's Intelligence:** Evidence-based results, click-to-element navigation. Instead of just stating an issue, DevLens points directly to the DOM node causing it.
*   **toast.log's Invisibility:** On-page overlays with zero UI chrome where possible. Injecting functional tools directly into the user's view without blocking their work.

## 2. Dual UI System

To achieve maximum performance and perfect isolation, DevLens employs a dual UI architecture:

*   **Layer 1 (On-page overlays):** Vanilla TypeScript + pure CSS injected into the host page via Shadow DOM.
    *   *Usage:* Element highlights, box model visualizations, rulers, grids, console toasts, crosshairs, tooltips.
    *   *Why:* Requires absolute 60fps performance without overhead. React is too heavy for rapid DOM mutations tracking mouse movements.
*   **Layer 2 (Sidebar HUD):** React + Tailwind CSS + shadcn/ui.
    *   *Usage:* Command palette, module panels, settings, deep data results.
    *   *Why:* shadcn uses Tailwind (global CSS) which would ordinarily conflict with host pages, but injecting the React root into a Shadow DOM isolates it completely. Allows rapid development of complex UI using standard ecosystem tools.

## 3. Design Tokens (DevLens's own theme)

The extension uses its own localized design system based on standard primitives but specifically tuned for a developer tooling environment.

### Colors (Dark theme — default)
*   **Background:** `hsl(0 0% 7%)` — near-black like SuperDev Pro for minimum eye strain.
*   **Surface:** `hsl(0 0% 11%)` — card/panel backgrounds.
*   **Surface-hover:** `hsl(0 0% 15%)` — interactive element hover states.
*   **Border:** `hsl(0 0% 18%)` — subtle dividers.
*   **Text-primary:** `hsl(0 0% 93%)` — high contrast readability.
*   **Text-secondary:** `hsl(0 0% 60%)` — standard labels and subtext.
*   **Text-muted:** `hsl(0 0% 40%)` — disabled states and minor hints.
*   **Accent-blue:** `hsl(217 91% 60%)` — links, primary actions, selections.
*   **Accent-green:** `hsl(142 71% 45%)` — success states, passing audits, standard logs.
*   **Accent-yellow:** `hsl(48 96% 53%)` — warnings, cautions, box-model borders.
*   **Accent-red:** `hsl(0 84% 60%)` — errors, critical issues, rulers.
*   **Accent-purple:** `hsl(262 83% 68%)` — AI/special features, grids.
*   **Accent-orange:** `hsl(25 95% 53%)` — medium severity, box-model margins.

### Colors (Light theme)
Light theme dynamically generates inverted versions of the dark tokens to maintain consistent contrast ratios while flipping the luminance.
*   **Background:** `hsl(0 0% 98%)`
*   **Surface:** `hsl(0 0% 95%)`
*   **Text-primary:** `hsl(0 0% 10%)`
*   *(Accents remain largely the same, adjusted slightly for contrast against white)*

### Typography
*   **Font-mono:** `'JetBrains Mono', 'Fira Code', monospace` (for CSS values, hex codes, DOM nodes, scraped data).
*   **Font-sans:** `'Inter', system-ui, sans-serif` (for labels, navigation, UI text).
*   **Font sizes:**
    *   `xs`: 11px (Tiny badges, tooltips)
    *   `sm`: 12px (Secondary text, log output)
    *   `base`: 13px (Standard body text)
    *   `md`: 14px (Subheaders, primary list items)
    *   `lg`: 16px (Module titles)
    *   `xl`: 18px (Major headers)
*   *Note:* Extension UI explicitly uses smaller base sizes than typical web apps to maximize information density.

### Spacing
Based on a strict 4px grid system:
*   `1`: 4px
*   `2`: 8px
*   `3`: 12px
*   `4`: 16px
*   `6`: 24px
*   `8`: 32px

### Border Radius
*   `sm`: 4px
*   `md`: 6px
*   `lg`: 8px
*   `xl`: 12px

## 4. Sidebar HUD Layout

The primary Layer 2 interface uses a fixed, predictable layout that maximizes vertical space for data.

```text
┌─────────────────────────────┐
│  ⌘K Command Bar             │ ← Always visible at top
├─────────────────────────────┤
│  Module Header              │ ← Active module name + back button
├─────────────────────────────┤
│                             │
│  Module Content Area        │ ← Scrollable, module-specific
│  (varies per module)        │
│                             │
├─────────────────────────────┤
│  Status Bar                 │ ← Quick info (element count, etc.)
├─────────────────────────────┤
│  📦📦📦📦📦📦📦📦📦📦📦│ ← Module Quick Bar (14 icons)
└─────────────────────────────┘
```

*   **Default sidebar width:** 360px
*   **Resizable bounds:** 280px (min) – 500px (max)
*   **Position:** Right side of the viewport (user configurable to left).

## 5. shadcn/ui Component Map

Complete mapping of shadcn/ui React components to specific DevLens features.

### Core Shell
*   **Command (cmdk)** → Command Palette (search all features, launch modules, recent actions).
*   **Sheet** → Side panel container (when not using Chrome Side Panel API natively).
*   **Resizable** → Sidebar width adjustment.
*   **Tabs** → Module section navigation (e.g., switching between 'Computed' and 'Source').
*   **ToggleGroup** → Module Quick Bar (bottom icons for switching tools).
*   **Dialog** → Settings, preferences, and permissions requests.
*   **Switch** → Feature toggles (e.g., auto-start, dark mode).
*   **Sonner** → Sidebar-level notifications (e.g., 'Settings saved' — *not* used for on-page toasts).

### M1: CSS Inspector
*   **Accordion** → Collapsible CSS property groups (Layout, Typography, Colors, Effects).
*   **Input** → Property value inline editing.
*   **Popover** → Autocomplete suggestions for CSS values.
*   **Badge** → Color swatches next to hex/rgb values.
*   **Breadcrumb** → DOM path visualization (`html > body > div.container > h1`).
*   **ToggleGroup** → Computed vs Source CSS toggle.
*   **ScrollArea** → Managing long, overflowing CSS property lists.

### M3: Code Exporter
*   **ToggleGroup** → HTML+CSS / Tailwind / React / Vue format selector.
*   **ScrollArea** → Code preview container with syntax highlighting.
*   **Button** → Copy to clipboard action.
*   **Sonner** → 'Copied!' confirmation toast.

### M4: Color Tools
*   **Card** → Color palette cards displaying HEX, RGB, and HSL.
*   **Accordion** → Grouping colors by semantic role (backgrounds, text, borders, accents).
*   **Select** → Export format drop-down (CSS variables / JSON / Tailwind).
*   **Button** → Copy / Download actions.

### M5: Typography
*   **Table** → Font list detailing family, weight, size, line-height.
*   **Command** → Google Fonts search interface (filtering 1100+ fonts).
*   **Button** → Download font file action.

### M7: Asset Extractor
*   **Card** → Image thumbnail grid items.
*   **Dialog** → Full-size image preview modal.
*   **Checkbox** → Multi-select inputs for batch downloading.
*   **Select** → Filtering assets by type (img / svg / bg / lottie / video).
*   **Badge** → Asset count indicators per type category.
*   **Button** → Download individual / Download ZIP actions.

### M8: Page Capture
*   **ToggleGroup** → Viewport / Full Page / Element capture mode selector.
*   **Dialog** → Preview modal before finalizing download.
*   **Select** → Output format (PNG / JPG / WebP / PDF).

### M9: Design System
*   **Card** → Design token visualizers (spacing blocks, shadows, border-radii examples).
*   **Select** → System export format (CSS vars / Tailwind config / JSON).
*   **Table** → Typography scale representation.

### M10: SEO & Accessibility
*   **Accordion** → Expandable audit result groups (Issues, Passed, N/A).
*   **Badge** → Severity levels (critical/warning/info) mapped with color variants.
*   **Progress** → Overall score visualization (0-100 bars).
*   **Button** → Click-to-element action (scrolls host page and highlights flagged DOM node).
*   **Alert** → Critical accessibility blocker warnings.

### M11: Tech Stack
*   **Table** → Technology list showing name, category, and confidence level.
*   **Collapsible** → Evidence dropdown (why exactly a tech was detected).
*   **Badge** → Category tags (e.g., 'Analytics', 'Framework').
*   **Alert** → Known vulnerability warnings tied to detected versions.

### M12: Debug & Monitor
*   **ToggleGroup** → Console log filter tabs (All / Errors / Warnings / Info / Network).
*   **Collapsible** → Expandable deep log objects/arrays.
*   **Table** → Network requests view (URL, method, status, timing).
*   **Badge** → Log type indicators.

### M14: Utilities
*   **Table + Input** → Cookie editor matrix (editable key-value pairs).
*   **Accordion** → JSON tree viewer for structured data.
*   **Card** → QR code display container.
*   **Input** → CSS selector field for basic scraping.
*   **Table** → Scraped data preview matrix.

## 6. Layer 1: On-Page Overlay Specs

These elements exist outside of React. They are built with highly optimized Vanilla CSS/TS injected into the host page to prevent jank.

### Element Highlight
*   **Blue border:** `2px solid hsl(217 91% 60%)`
*   **Background:** `hsla(217 91% 60% / 0.08)`
*   **Label tooltip:** Displays element tag + dimensions (e.g., `div.hero 1200x400`).
*   **Tooltip Position:** Floating above the element's top-left corner (flips inside if at viewport edge).
*   **Font:** `11px monospace`, white text on dark pill background.

### Box Model Overlay
*   **Margin:** `hsla(25 95% 53% / 0.3)` — orange
*   **Border:** `hsla(48 96% 53% / 0.3)` — yellow
*   **Padding:** `hsla(142 71% 45% / 0.3)` — green
*   **Content:** `hsla(217 91% 60% / 0.3)` — blue
*   **Labels:** Small pixel values centered at each side of the respective box layers.

### Ruler
*   **Color:** `hsl(0 84% 60%)` (Accent-red)
*   **Style:** `1px dashed line`
*   **Distance label:** Centered directly on the line, white text on dark background pill.

### Grid Overlay
*   **Color:** `hsla(262 83% 68% / 0.15)`
*   **Column lines:** `1px solid hsla(262 83% 68% / 0.4)`
*   **Configurable Parameters:** columns count, gutter width, max-width.

### Toast Notifications (Console Monitor)
*   **Background:** `hsl(0 0% 10%)`
*   **Border-left:** `4px solid` (Green=log, Blue=info, Yellow=warn, Red=error)
*   **Font:** `12px monospace`
*   **Position:** Bottom-right of the viewport.
*   **Max-width:** `400px`
*   **Animation:** Slide in from right (`300ms ease-out`).
*   **Auto-dismiss:** 8s for standard logs, completely sticky for errors.

### Color Picker Crosshair
*   **Cursor:** `2px white` crosshair with `1px dark` outline for contrast on any background.
*   **Magnified pixel preview:** A 100x100px floating grid showing the exact 11x11 pixels surrounding the cursor.
*   **Selected color preview:** A solid circle below the crosshair showing the currently targeted color.

## 7. Command Palette Specification

The Command Palette is the nervous system of DevLens, offering instant access to all features.

*   **Trigger:** `Ctrl+Shift+K` (Windows/Linux) / `⌘+Shift+K` (macOS).
    *   *Note:* Cannot use `Ctrl+K` directly as browsers reserve it for the address bar/search.
*   **Behavior:** Opens instantly (`<50ms` latency). Uses fuzzy search indexing across all commands, actions, and settings.
*   **Categories:** Modules, Actions, Settings, Recent.
*   **Anatomy of a Command:** `[Icon] Label [Shortcut Hint] [Category tag]`
*   **Top level commands:**
    *   Launch CSS Inspector
    *   Launch Code Exporter
    *   Launch Color Tools
    *   Launch Typography
    *   Launch Asset Extractor
    *   Launch Page Capture
    *   Launch Design System
    *   Launch SEO & Accessibility
    *   Launch Tech Stack
    *   Launch Debug & Monitor
    *   Launch Utilities
    *   Action: Toggle Ruler
    *   Action: Toggle Grid
    *   Action: Clear Console
*   **State:** Recent commands are always populated first when opened empty.
*   **Exit:** `Escape` key closes the palette instantly.

## 8. Keyboard Shortcuts Map

Keyboard navigation is treated as a first-class citizen for power users.

| Shortcut | Action | Context |
| :--- | :--- | :--- |
| `Ctrl+Shift+E` | Toggle DevLens sidebar | Global |
| `Ctrl+Shift+K` | Open command palette | Global |
| `Escape` | Close current panel/overlay/palette | Any active DevLens UI |
| `1` through `9` | Quick switch to specific module | Sidebar focused |
| `I` | Activate CSS Inspector mode | Global (when DevLens active) |
| `C` | Activate Color Picker | Global (when DevLens active) |
| `F` | Activate Font Inspector | Global (when DevLens active) |
| `S` | Take screenshot | Global (when DevLens active) |
| `T` | Toggle toast monitor | Global (when DevLens active) |
| `R` | Toggle ruler overlay | Global (when DevLens active) |
| `G` | Toggle grid overlay | Global (when DevLens active) |
| `M` | Activate measure mode | Global (when DevLens active) |
| `D` | Activate delete element mode | Global (when DevLens active) |
| `P` | Activate pick element mode | Global (when DevLens active) |

*(Note: Module-specific shortcuts, like 'copy CSS' or 'export to Tailwind', will be surfaced within the specific module UI).*

## 9. Icons

DevLens standardizes on **Lucide** icons (native to the shadcn ecosystem).
*   **Size Constraints:** Consistent 18px size in the bottom Module Quick Bar; 16px in standard menus and buttons.

**Core Module Icon Mapping:**
*   Inspect = `MousePointer`
*   Colors = `Palette`
*   Fonts = `Type`
*   Layout = `LayoutGrid`
*   Assets = `Image`
*   Capture = `Camera`
*   Design System = `Paintbrush`
*   SEO & A11y = `Search`
*   Tech Stack = `Layers`
*   Debug = `Bug`
*   Utilities = `Wrench`
*   Code Export = `Code`
*   Command = `Command`
*   DOM Explorer = `Box`

## 10. Animations & Transitions

UI motion is highly constrained to avoid feeling sluggish. Strict timing curves apply:

*   **Sidebar open/close:** `200ms ease-out` (`translate-x`).
*   **Panel switch (Internal):** `150ms fade`.
*   **Toast appear:** `300ms ease-out` (`translate-x` from the right edge).
*   **Toast dismiss:** `200ms ease-in` (`translate-x` to the right edge).
*   **Element highlight (Layer 1):** `0ms` (Instant — must be 60fps tracking mouse, no CSS transitions allowed).
*   **Command palette:** `100ms scale(0.98 -> 1) + fade`.
*   **Accessibility:** Must explicitly respect `@media (prefers-reduced-motion: reduce)` by disabling all scaling/translating and reverting to instant opacity swaps.

## 11. Accessibility (of DevLens itself)

DevLens must set the standard for a11y, even as a dev tool.

*   **Keyboard Navigation:** 100% of interactive elements in Layer 2 must be reachable via `Tab`.
*   **Focus Ring:** `2px solid accent-blue` with a `2px offset`. Native outline removed.
*   **ARIA Labels:** Strictly required on all icon-only buttons (e.g., Quick Bar module icons).
*   **Command Palette Role:** Explicitly set to `role='dialog'` and traps focus.
*   **Screen Readers:** Live announcements triggered for dynamic audit results or major state changes.
*   **Contrast:** All text (including code blocks) must meet WCAG AA (`4.5:1` minimum).
*   **Logical Tab Order:** Command bar (Top) → Module content area (Middle) → Module Quick Bar (Bottom).

## 12. Responsive Sidebar

DevLens adapts to the user's viewport to ensure it doesn't break the layout of the host page being debugged.

*   **Page width < 600px:** Sidebar morphs into a full-width overlay sheet (covers the screen, dismissable via `Esc` or close button).
*   **Page width 600px - 1200px:** Sidebar defaults and locks to `300px` width.
*   **Page width > 1200px:** Sidebar defaults to standard `360px` width.
*   **User Adjustments:** Within supported viewports, the user can freely resize the sidebar between `280px` and `500px`.
*   **Persistence:** Size and Left/Right position preferences are saved to `chrome.storage.local` and instantly applied on the next page load.
