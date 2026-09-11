---
name: devlens-modules
description: >-
  Complete specification for all 14 DevLens feature modules including browser
  APIs, input/output contracts, UI components, and implementation guidance.
---

# DevLens Feature Modules Specification

This document provides the complete technical specification for all 14 DevLens feature modules. It details the 69 non-AI features across the browser extension, intended as a primary reference for both human developers and LLM agents building the application.

## Module Template

Every module follows this standard template:

### M[N]: [Module Name]
- **Purpose:** One-line description
- **Features:** Numbered list of features in this module
- **Priority:** P0/P1/P2 per feature
- **Browser APIs:** Exact APIs used
- **Input:** What the user provides
- **Output:** What the user gets back
- **UI Layer:** Where it renders (Sidebar / On-page / Both)
- **shadcn Components:** Which shadcn components are used
- **Dependencies:** Other modules this depends on
- **Complexity:** Easy / Medium / Hard
- **Key Implementation Notes:** Technical hints and patterns

---

## The Modules

### M1: CSS Inspector & Editor
- **Purpose:** Allows developers to inspect, extract, and live-edit CSS properties of any element on the page.
- **Features:**
  1. Hover-to-inspect CSS (P0)
  2. Live CSS editing with autocomplete (P0)
  3. Real CSS extraction (authored not computed) (P1)
  4. Pseudo-element inspection (P1)
  5. Computed vs Source CSS toggle (P2)
- **Priority:** P0/P1/P2
- **Browser APIs:** `window.getComputedStyle()`, `element.style`, `document.styleSheets`, `CSSStyleSheet.cssRules`, `element.matches()`, `document.querySelectorAll()`, `MutationObserver`
- **Input:** User hovers/clicks elements on page; user types CSS property/value in editor.
- **Output:** Visual highlight on hover, CSS properties in sidebar, editable values, exportable CSS string, real-time DOM updates.
- **UI Layer:** Both (Layer 1 for highlight + tooltip, Layer 2 sidebar for property panel)
- **shadcn Components:** `Tabs`, `Input`, `Label`, `ScrollArea`, `HoverCard`, `Badge`
- **Dependencies:** None
- **Complexity:** Medium-Hard
- **Key Implementation Notes:**
  - **Real CSS Extraction:** Iterating through `document.styleSheets` is required to find authored CSS rather than just computed values. Use a `try/catch` block as accessing cross-origin stylesheets will throw a `DOMException` (SecurityError).
  - Loop through `sheet.cssRules` and match rules against the selected element using `element.matches(rule.selectorText)`.
  - Filter the matched CSS to only rules affecting the selected element. Preserve original class names and avoid expanding shorthands unless necessary.
  - **Variables:** Resolve CSS variables via `getComputedStyle(element).getPropertyValue('--var-name')` but ensure the original `var(--var-name)` is shown in the source view for accuracy.
  - **Live Edits:** Use `MutationObserver` configured with `{ attributes: true, attributeFilter: ['style', 'class'] }` to track live edits made by other scripts on the page and reflect them in the DevLens UI.
  - **Pseudo-elements:** Inspect pseudo-elements using `window.getComputedStyle(element, '::before')` or `::after`.

### M2: DOM Manipulator
- **Purpose:** Modify page content visually on the fly for testing or mockups without writing code.
- **Features:**
  1. In-place text editing (P0)
  2. Element deletion (P1)
  3. Element move/drag (P1)
  4. Image swapping (P2)
  5. Hide banners/popups/widgets (P1)
- **Priority:** P0/P1/P2
- **Browser APIs:** `element.contentEditable`, `element.remove()`, `element.style.display`, `PointerEvent` (`pointerdown`, `pointermove`, `pointerup`), `element.setAttribute('src')`
- **Input:** User clicks/drags elements on the page, types new text, or selects replacement images.
- **Output:** Modified page structure and content (temporary, reverts on reload).
- **UI Layer:** Layer 1 only (on-page interaction via overlays or direct element manipulation).
- **shadcn Components:** `ContextMenu` (for right-click actions like Delete/Hide), `Dialog` (for image swap URL input).
- **Dependencies:** None
- **Complexity:** Easy-Medium
- **Key Implementation Notes:**
  - **Text Editing:** Set `element.contentEditable = 'true'` and `element.focus()`. Add event listeners for `blur` or `keydown` (Enter/Escape) to save/cancel edits and set `contentEditable = 'false'`.
  - **Drag and Drop:** Implement custom drag using pointer events. On `pointerdown`, record initial mouse position and element's original transform. On `pointermove`, calculate delta and apply via `element.style.transform = 'translate(x, y)'`. Ensure `z-index` is elevated during drag.
  - **State Management:** Store original states (inner text, display property, position, parent node) in a local history stack array to implement an "Undo" feature (`Ctrl+Z`).
  - **Image Swapping:** Provide a UI to enter a new URL, then simply `element.src = newUrl`. Maintain original `width`/`height` attributes to prevent layout shifts if desired.

### M3: Code Exporter
- **Purpose:** Export selected elements and their styles into ready-to-use code snippets.
- **Features:**
  1. Export HTML+CSS file (P0)
  2. Export to CodePen (P1)
  3. Export Tailwind CSS (P1)
  4. Export React JSX (P1)
  5. Export Vue SFC (P2)
- **Priority:** P0/P1/P2
- **Browser APIs:** DOM serialization (`element.outerHTML`), Clipboard API (`navigator.clipboard.writeText`), `window.open()`
- **Input:** Selected element(s) from M1 via UI trigger.
- **Output:** Clean code string in chosen format, copied to clipboard or opened in an external tool like CodePen.
- **UI Layer:** Layer 2 (sidebar code preview panel with syntax highlighting).
- **shadcn Components:** `Tabs`, `Button`, `Select`, `ScrollArea`, `Toast` (for copy confirmation).
- **Dependencies:** M1 (CSS Inspector — needs extracted CSS to work properly)
- **Complexity:** Hard
- **Key Implementation Notes:**
  - **HTML+CSS Serialization:** Recursively clone the selected node. Clean up injected DevLens classes or attributes. Serialize to string using `clone.outerHTML`.
  - **Tailwind Conversion:** This is complex. Build or integrate a mapping dictionary (property→class lookup table). For example, `display: flex` maps to `flex`, `margin-top: 1rem` maps to `mt-4`. You may need to round values to the nearest Tailwind spacing scale.
  - **React JSX:** Parse the HTML string and convert attributes: `class` to `className`, `for` to `htmlFor`. Convert inline `style="color: red; margin-top: 10px"` to objects `style={{ color: 'red', marginTop: '10px' }}`. Ensure self-closing tags like `<img>` or `<input>` are properly closed (`<img />`).
  - **Vue SFC:** Wrap the HTML in a `<template>` block and the extracted CSS in a `<style scoped>` block.
  - **CodePen Export:** Create a hidden `<form>` element with `method="POST"` and `action="https://codepen.io/pen/define"`. Add an `<input type="hidden" name="data">` containing a JSON stringified object with `html`, `css`, and `js` keys, then call `form.submit()`.

### M4: Color Tools
- **Purpose:** Sample, extract, and organize color palettes from the current page.
- **Features:**
  1. Color Eyedropper (HEX/RGB/HSL) (P0)
  2. Full palette extraction grouped by role (P1)
  3. Export as CSS variables (P1)
  4. Export as JSON (P2)
- **Priority:** P0/P1/P2
- **Browser APIs:** `EyeDropper` API (`new EyeDropper()`), `window.getComputedStyle()`, `CanvasRenderingContext2D.getImageData()` (fallback for eyedropper)
- **Input:** User activates eyedropper crosshair or clicks 'Extract palette'.
- **Output:** Color values in various formats, organized palette UI, exportable text format.
- **UI Layer:** Both (Layer 1 for crosshair/picker if custom, Layer 2 for palette display and management)
- **shadcn Components:** `Card`, `Tooltip`, `Button`, `DropdownMenu` (for export options).
- **Dependencies:** None
- **Complexity:** Medium
- **Key Implementation Notes:**
  - **Eyedropper:** Use the native `EyeDropper` API if available (`if ('EyeDropper' in window)`). It returns an sRGBHex string (`#RRGGBB`).
  - **Fallback:** If `EyeDropper` is not supported (e.g., Firefox), you cannot take accurate cross-origin pixel samples natively without a background script `captureVisibleTab`, which is heavier. Rely on computed styles for fallback.
  - **Palette Extraction:** Traverse all elements using `document.querySelectorAll('*')`. Collect `color`, `backgroundColor`, `borderColor`, `outlineColor` via `getComputedStyle`.
  - **Deduplication & Grouping:** Filter out transparent values (`rgba(0, 0, 0, 0)`). Deduplicate similar colors. Group by property type (Backgrounds, Text, Borders).
  - **Conversion:** Implement utility functions to convert between HEX, RGB, and HSL using standard color math formulas.

### M5: Typography Tools
- **Purpose:** Inspect and manipulate typography settings, and preview font replacements.
- **Features:**
  1. Font detection (family/weight/size/line-height) (P0)
  2. Font file download (P2)
  3. Google Fonts swap/preview (P1)
  4. Font usage mapping (P1)
- **Priority:** P0/P1/P2
- **Browser APIs:** `window.getComputedStyle()`, `document.fonts` (CSS Font Loading API), `fetch()`
- **Input:** User hovers element to detect, or selects a font from a list to swap.
- **Output:** Font details tooltip, downloadable font files, live preview of new font on the page.
- **UI Layer:** Both (Layer 1 for hover label, Layer 2 for font table/swap UI)
- **shadcn Components:** `Table`, `Select`, `Slider` (for testing sizes), `Badge`.
- **Dependencies:** None
- **Complexity:** Medium
- **Key Implementation Notes:**
  - **Detection:** `getComputedStyle(element).fontFamily` returns the resolved font stack. To check which font is *actually* being rendered, iterate through the stack and use `document.fonts.check('16px "Font Name"')` to verify if it's loaded and used.
  - **Font Download:** Parse `document.styleSheets` for `@font-face` rules. Extract the `src` URL. If it's a relative URL, resolve it against `window.location.origin`. Provide a download link. Note CORS restrictions may apply.
  - **Google Fonts Swap:** Fetch the list of available Google Fonts (cache this list). When a user selects a font, inject a `<link href="https://fonts.googleapis.com/css2?family=..." rel="stylesheet">` into the `<head>`.
  - Then, apply a CSS rule to the `html` or specific element: `* { font-family: 'Selected Font', sans-serif !important; }` to preview it.
  - **Usage Mapping:** Traverse the DOM and group elements by their computed `font-family` to show a breakdown of where different fonts are used.

### M6: Layout & Measurement
- **Purpose:** Visualize structure, spacing, and dimensions of page elements.
- **Features:**
  1. Box model visualization (P0)
  2. Page ruler/distance measurement (P1)
  3. Element outliner (flex/grid) (P1)
  4. Grid overlays (P2)
  5. Responsive viewport simulator (P1)
- **Priority:** P0/P1/P2
- **Browser APIs:** `element.getBoundingClientRect()`, `window.getComputedStyle()`, Canvas API, `iframe`
- **Input:** User hovers (box model) or activates measurement mode/rulers.
- **Output:** Visual overlays on page indicating margins, padding, borders, and pixel measurements.
- **UI Layer:** Both (Layer 1 for overlays, Layer 2 for responsive viewer controls)
- **shadcn Components:** `ToggleGroup` (for modes), `Slider`, `Card`.
- **Dependencies:** None
- **Complexity:** Medium-Hard
- **Key Implementation Notes:**
  - **Box Model Overlay:** Create 4 nested `div` elements appended to the body with absolute positioning and high `z-index`. Calculate dimensions using `getBoundingClientRect()` for the content box, and add `margin`, `padding`, `border` values from `getComputedStyle()`.
  - Apply distinct semi-transparent colors (e.g., Margin=Orange, Border=Yellow, Padding=Green, Content=Blue).
  - Update positions on `scroll` and `resize` events using `requestAnimationFrame` for performance.
  - **Ruler:** On click-and-drag, draw a line between the start point and current mouse position using an SVG `<line>` or an absolutely positioned `div` with calculated width and rotation (`Math.atan2`). Display a label with the distance in pixels (`Math.hypot(dx, dy)`).
  - **Responsive Simulator:** Hide the main body content and wrap the page in an `iframe` (or multiple iframes for side-by-side comparison). Adjust the `iframe` width/height to match device presets.
  - **Flex/Grid Outliner:** Query all elements, check `getComputedStyle(el).display`. If it includes `flex` or `grid`, add a specific outline class to them and inject a small label indicating layout direction or grid tracks.

### M7: Asset Extractor
- **Purpose:** Find, preview, and download all media assets from the page.
- **Features:**
  1. Image extraction (img tags) (P0)
  2. SVG extraction (inline + img) (P0)
  3. Lottie animation extraction (P2)
  4. Background image extraction (P1)
  5. Video extraction (P1)
  6. PDF extraction (P2)
  7. Batch ZIP download (P1)
- **Priority:** P0/P1/P2
- **Browser APIs:** `document.querySelectorAll()`, `window.getComputedStyle()`, `chrome.downloads` (if applicable), JSZip library
- **Input:** User opens the asset extractor panel.
- **Output:** Grid of discovered assets, downloadable individually or as a ZIP archive.
- **UI Layer:** Layer 2 (sidebar asset grid with thumbnails and filters)
- **shadcn Components:** `Card` (for grid items), `Checkbox`, `Button`, `Tabs` (to filter types).
- **Dependencies:** None
- **Complexity:** Medium
- **Key Implementation Notes:**
  - **Images & Videos:** Use `document.querySelectorAll('img, video, source')`. Extract `src` attributes. Resolve relative URLs using `new URL(el.src, window.location.href)`.
  - **Background Images:** Traverse all elements, check `getComputedStyle(el).backgroundImage`. Use regex `/url\(['"]?(.*?)['"]?\)/` to extract URLs.
  - **Inline SVGs:** Find all `<svg>` elements. Serialize using `new XMLSerializer().serializeToString(svgNode)`. Base64 encode it `btoa()` to create a downloadable data URI.
  - **Lottie:** Look for `<lottie-player>` tags, or inspect scripts/network requests (if possible via background script) for `.json` files that look like Lottie animations.
  - **Batch ZIP:** Include `jszip` dependency. Fetch each asset URL as a `blob` (`fetch(url).then(r => r.blob())`), add to the zip instance (`zip.file(filename, blob)`), and generate the archive (`zip.generateAsync()`).

### M8: Page Capture
- **Purpose:** Take screenshots or clones of the page or specific elements.
- **Features:**
  1. Viewport screenshot (P0)
  2. Full-page screenshot (scrolled+stitched) (P1)
  3. Element screenshot (exact bounds) (P1)
  4. Page cloning to single HTML file (P2)
  5. PDF saving (P1)
- **Priority:** P0/P1/P2
- **Browser APIs:** `chrome.tabs.captureVisibleTab()`, `window.scrollTo()`, Canvas API, `DOMSerializer`, `window.print()`
- **Input:** User selects a capture mode.
- **Output:** PNG/JPG/WebP image file, consolidated HTML file, or PDF.
- **UI Layer:** Layer 2 (sidebar preview + action buttons)
- **shadcn Components:** `Button`, `DropdownMenu` (for format selection).
- **Dependencies:** None
- **Complexity:** Hard
- **Key Implementation Notes:**
  - **Viewport Capture:** Simple call to `chrome.tabs.captureVisibleTab()` in the background script.
  - **Full-page Screenshot:** Requires a coordinator script. Hide fixed/sticky elements (temporarily change their position to absolute). Scroll down by viewport height iteratively. Wait for rendering (`setTimeout` or `requestAnimationFrame`). Capture each segment. Draw them sequentially onto a large offscreen Canvas. Restore element positions.
  - **Element Capture:** Perform a full-page capture (or use `html2canvas` library as an alternative, though less accurate), then use Canvas `drawImage` with cropping parameters matching the element's `getBoundingClientRect()`.
  - **Page Cloning:** Serialize the DOM. Find all linked CSS `<link rel="stylesheet">`, fetch their contents, and inline them into `<style>` tags. Convert all relative `src` and `href` attributes to absolute URLs. Remove `<script>` tags to prevent execution in the static clone.
  - **PDF:** Call `window.print()`. To improve results, inject a specific `@media print` stylesheet temporarily to hide extension UI and optimize layout before printing.

### M9: Design System Capturer
- **Purpose:** Automatically extract a site's foundational design tokens.
- **Features:**
  1. Full design system capture (palette+fonts+spacing+shadows+radii) (P1)
  2. Export as CSS variables (P1)
  3. Export as Tailwind config (P1)
  4. Export as JSON (P1)
- **Priority:** P0/P1/P2
- **Browser APIs:** DOM Traversal, `window.getComputedStyle()`
- **Input:** User clicks 'Capture design system'.
- **Output:** Organized design tokens presented in UI, exportable in multiple formats.
- **UI Layer:** Layer 2 (sidebar token display grouped by category)
- **shadcn Components:** `Accordion` (for categories), `Badge`, `Button`, `Textarea` (for export code).
- **Dependencies:** M4 (Color Tools), M5 (Typography)
- **Complexity:** Hard
- **Key Implementation Notes:**
  - **Traversal Strategy:** Iterate through `document.body.getElementsByTagName('*')`. Avoid SVG interiors to save time.
  - **Data Collection:**
    - Spacing: Extract `margin` and `padding` values. Convert to consistent units (e.g., px).
    - Shadows: Extract `box-shadow`.
    - Radii: Extract `border-radius`.
  - **Frequency Analysis:** Count occurrences of each value. Filter out anomalies or one-off values (e.g., must appear more than N times to be considered a token).
  - **Sorting:** Sort spacing and radii numerically. Sort colors by luminosity or hue.
  - **Export Generation:**
    - CSS: Generate `:root { --space-1: 4px; --space-2: 8px; ... }`
    - Tailwind: Generate a `tailwind.config.js` snippet extending `theme: { colors: {...}, spacing: {...} }`.

### M10: SEO & Accessibility Auditor
- **Purpose:** Evaluate the current page against SEO and a11y best practices.
- **Features:**
  1. SEO audit (headings/meta/canonical) (P1)
  2. Structured data analysis (P1)
  3. Broken link checker (P1)
  4. Accessibility audit (ARIA/roles/alt) (P1)
  5. WCAG contrast checking (P1)
  6. Focus path visualization (P2)
- **Priority:** P0/P1/P2
- **Browser APIs:** `document.querySelectorAll()`, `fetch()`, `window.getComputedStyle()`
- **Input:** User clicks 'Run Audit'.
- **Output:** List of issues grouped by severity (Error, Warning, Pass), click-to-highlight elements.
- **UI Layer:** Layer 2 (sidebar results list) + Layer 1 (highlight flagged elements on page)
- **shadcn Components:** `Alert`, `Accordion`, `Progress`, `Badge` (for severities).
- **Dependencies:** None
- **Complexity:** Medium-Hard
- **Key Implementation Notes:**
  - **SEO Checks:**
    - Verify exactly one `<h1>`.
    - Check heading order logic (e.g., an `<h3>` should not immediately follow an `<h1>`).
    - Verify `<title>` length (50-60 chars) and `<meta name="description">` length.
    - Check for `<link rel="canonical">`.
  - **Accessibility Checks:**
    - Find `<img>` without `alt` attribute.
    - Find `<button>` or `<a>` without text content or `aria-label`.
  - **Contrast Ratio:** Use `getComputedStyle` for text color and background color. Calculate relative luminance: `L = 0.2126 * R + 0.7152 * G + 0.0722 * B` (where RGB values are linearized). Contrast ratio = `(L1 + 0.05) / (L2 + 0.05)`. Warn if below 4.5:1 (AA) or 7:1 (AAA). Note: Handling transparent backgrounds requires traversing up the DOM tree to compute the true background color.
  - **Broken Links:** Collect all unique `href` from `<a>` tags. Filter out `javascript:` or `mailto:`. Send a `HEAD` request via `fetch(url, { method: 'HEAD', mode: 'no-cors' })`. Flag 404s.

### M11: Tech Stack Analyzer
- **Purpose:** Identify the frameworks, CMS, and libraries used to build the site.
- **Features:**
  1. Framework/CMS/library detection (P1)
  2. Detection evidence display (P1)
  3. HTTP headers analysis (P1)
  4. Vulnerability scanning (P2)
  5. DNS/domain info (P2)
- **Priority:** P0/P1/P2
- **Browser APIs:** DOM inspection, `chrome.webRequest.onHeadersReceived` (background script), `fetch()`
- **Input:** User opens the Tech Stack tab.
- **Output:** List of detected technologies with icons, confidence levels, and evidence.
- **UI Layer:** Layer 2 (sidebar list)
- **shadcn Components:** `Card`, `HoverCard` (for evidence), `Avatar` (for tech icons).
- **Dependencies:** None
- **Complexity:** Hard
- **Key Implementation Notes:**
  - **Signature Database:** Build a JSON dictionary mapping technologies to signatures.
    - Global variables: `window.React`, `window.Vue`, `window.jQuery`.
    - DOM markers: `<div id="__next">` (Next.js), `data-reactroot` (React), `ng-version` (Angular).
    - Meta tags: `<meta name="generator" content="WordPress">`.
    - Script tags: Regex match `src` attributes for `cdn.jsdelivr.net/npm/bootstrap`.
  - **HTTP Headers:** Requires a background script listening to `chrome.webRequest.onHeadersReceived`. Check headers like `X-Powered-By` (e.g., Express, PHP) and `Server` (e.g., nginx, Cloudflare). Send this data to the content script/sidebar via message passing.
  - **Evidence:** Store the reason a technology was detected (e.g., "Found window.__NUXT__ variable") and display it in the UI to build trust.

### M12: Debug & Monitor
- **Purpose:** Monitor console outputs, network requests, and runtime errors in a clean UI.
- **Features:**
  1. Console log→toast (P1)
  2. Console error/warn/info display (P1)
  3. Network request tracking (P1)
  4. Error tracing (file+line) (P1)
  5. DOM error detection (P2)
  6. One-click error Google search (P2)
- **Priority:** P0/P1/P2
- **Browser APIs:** Monkey-patching `console`, `PerformanceObserver`, `window.addEventListener('error')`, `MutationObserver`
- **Input:** User activates debug mode/monitoring.
- **Output:** Toasts on page for new logs, list view in sidebar with filters (All, Errors, Warnings).
- **UI Layer:** Both (Layer 1 for toast notifications, Layer 2 for log list)
- **shadcn Components:** `Toast` / `Toaster`, `Tabs` (for filters), `Table`.
- **Dependencies:** None
- **Complexity:** Medium-Hard
- **Key Implementation Notes:**
  - **Console Patching:** **CRITICAL:** Content scripts execute in an isolated world and cannot override the page's `console`. You must inject a `<script>` tag into the page's DOM (Main World) at `document_start` to monkey-patch `console.log`, `console.error`, etc.
    - Keep a reference to the original: `const originalLog = console.log;`
    - Override: `console.log = function(...args) { originalLog.apply(console, args); window.postMessage({ type: 'DEVLENS_LOG', payload: args }, '*'); }`
  - **Error Catching:** Add listeners for `error` and `unhandledrejection` in the injected script. Extract message, filename, lineno, and stack trace.
  - **Network Tracking:** Use `new PerformanceObserver((list) => { ... }).observe({ entryTypes: ['resource'] })` to track network requests, load times, and payload sizes without needing heavy background `webRequest` permissions.
  - **DOM Errors:** Use `MutationObserver` to watch for newly added `<img>` elements. Check if `img.naturalWidth === 0` after load to flag broken images.

### M13: Command Center
- **Purpose:** Quick keyboard-driven navigation and tool execution (like Spotlight or Raycast).
- **Features:**
  1. Command palette with fuzzy search (P0)
  2. Keyboard shortcuts system (P1)
  3. Tool launcher (P0)
- **Priority:** P0/P1/P2
- **Browser APIs:** `window.addEventListener('keydown')`
- **Input:** User presses `Ctrl+Shift+K` (or `Cmd+Shift+K`) and types queries.
- **Output:** Centered overlay with a searchable list of commands. Pressing Enter executes the action.
- **UI Layer:** Layer 2 (Centered Modal / Command Palette overlay)
- **shadcn Components:** `Command`, `CommandInput`, `CommandList`, `CommandGroup`, `CommandItem`, `Dialog`.
- **Dependencies:** All other modules (commands must be registered from each module).
- **Complexity:** Medium
- **Key Implementation Notes:**
  - **Event Listener:** Attach a global `keydown` listener for the activation shortcut. Ensure it `preventDefault()` if triggered to avoid browser conflicts.
  - **Registry:** Create a central singleton registry where modules can register their actions upon initialization. e.g., `CommandRegistry.register({ id: 'm1-inspect', label: 'Inspect CSS', icon: EyeIcon, action: () => M1.start() })`.
  - **Fuzzy Search:** Use the `cmdk` library (which powers shadcn `Command`) for built-in filtering, or integrate `Fuse.js` for more advanced fuzzy matching across titles and aliases.
  - **State:** Use `chrome.storage.session` to remember the most recently used commands and boost their ranking in the default empty-search view.

### M14: Utilities
- **Purpose:** A collection of small, handy developer tools that don't fit into other modules.
- **Features:**
  1. Cookie editor (P1)
  2. JSON formatter (P1)
  3. API tester (P2)
  4. QR code generator (P2)
  5. Sticky notes (P2)
  6. Data scraper (P1)
  7. Page-to-Markdown (P2)
  8. UTM builder (P2)
  9. Cache clearing (P1)
- **Priority:** P0/P1/P2
- **Browser APIs:** `chrome.cookies`, `JSON.parse/stringify`, `fetch()`, `chrome.browsingData`
- **Input:** Varies (URLs, JSON strings, CSS selectors).
- **Output:** Varies (Formatted text, modified cookies, cleared cache).
- **UI Layer:** Layer 2 (sidebar, utilities menu with sub-views)
- **shadcn Components:** `Tabs`, `Input`, `Textarea`, `Button`, `Table`.
- **Dependencies:** None
- **Complexity:** Easy-Medium (per utility)
- **Key Implementation Notes:**
  - **Cookie Editor:** Requires `cookies` permission. Use `chrome.cookies.getAll({ url: window.location.origin })` to list. Provide inputs to edit value/expiration, then call `chrome.cookies.set()`.
  - **JSON Formatter:** Provide a textarea. On paste, `try { const obj = JSON.parse(val); return JSON.stringify(obj, null, 2); }`. Use a syntax highlighter component for display.
  - **API Tester:** Simple UI with Method dropdown, URL input, and body textarea. Note that `fetch()` calls from the content script are subject to the page's CORS policy. For cross-origin requests, relay the request to the background script.
  - **Data Scraper:** User inputs a selector (e.g., `.product-title`). Run `document.querySelectorAll()`. Extract `innerText` or `href` attributes, format as CSV or JSON, and provide a download link.
  - **Cache Clearing:** Use `chrome.browsingData.remove({ origins: [window.location.origin] }, { cache: true, cookies: true, localStorage: true })`. Requires `browsingData` permission.
