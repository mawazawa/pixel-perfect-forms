# 2025 Browser/WASM Support Snapshot

██████╗ ███████╗██████╗ ██████╗  ██████╗ ██████╗ ██╗   ██╗███████╗██████╗
██╔══██╗██╔════╝██╔══██╗██╔══██╗██╔═══██╗██╔══██╗╚██╗ ██╔╝██╔════╝██╔══██╗
██████╔╝█████╗  ██████╔╝██████╔╝██║   ██║██████╔╝ ╚████╔╝ █████╗  ██████╔╝
██╔═══╝ ██╔══╝  ██╔══██╗██╔══██╗██║   ██║██╔══██╗  ╚██╔╝  ██╔══╝  ██╔══██╗
██║     ███████╗██████╔╝██║  ██║╚██████╔╝██║  ██║   ██║   ███████╗██████╔╝
╚═╝     ╚══════╝╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚══════╝╚═════╝
  
                                                    RESEARCH.md

Date: 2025‑08‑21
Scope: Screen‑first, print‑accurate Rust+WASM forms. Focus on APIs impacting
  rendering fidelity, proofing, and perf.

---

## CSS Houdini Paint API

- __Status__: Stable in Chromium; not in Firefox; Safari disabled by default
  (including TP/26).
- __Implication__: Treat as progressive enhancement (rulers/grid). Provide
  canvas/DOM fallback.
- __Refs__:
  - MDN: [CSS Houdini](https://developer.mozilla.org/en-US/docs/Web/API/CSS_Houdini_APIs)
  - W3C: [CSS Painting API](https://www.w3.org/TR/css-paint-api-1/)
  - Is Houdini Ready Yet: <https://ishoudinireadyyet.com/>

## CSS Anchor Positioning

- __Status__: Chromium stable (125+); Safari 26 stable; Firefox: no support.
- __Implication__: Great for tethered UI (tooltips/menus). Keep fallback
  positioning for Firefox.
- __Refs__:
  - MDN: <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_anchor_positioning>
  - Can I use: <https://caniuse.com/css-anchor-positioning>
  - Chrome Docs: <https://developer.chrome.com/docs/css-ui/anchor-positioning-api>

## Popover API

- __Status__: Cross‑browser stable by early 2025 (MDN), with attribute+JS API.
- __Implication__: Prefer native popovers for proofing menus/tooltips; use
  `dialog` or custom fallback if absent.
- __Refs__:
  - MDN Overview: <https://developer.mozilla.org/en-US/docs/Web/API/Popover_API>
  - MDN HTMLElement.popover: <https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/popover>

## View Transitions (Single‑document)

- __Status__: Chromium stable (111+); Safari 18+; Firefox: not supported.
- __Implication__: Optional polish for mode transitions; ensure non‑transition path.
- __Refs__:
  - MDN: <https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API>
  - Can I use: <https://caniuse.com/view-transitions>

## CSS Container Queries (Size)

- __Status__: Stable across Chromium, Safari (16+), Firefox (110+).
- __Implication__: Safe to use for responsive, component‑scoped UI.
- __Refs__:
  - MDN: <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_containment/Container_queries>
  - Can I use: <https://caniuse.com/css-container-queries>

## WebGPU

- __Status__: Chromium stable (113+); Safari 26 stable; Firefox 141 (Windows)
  enabled; other platforms progressing.
- __Implication__: Candidate for heavy compute/diff in future; start with
  WASM+CPU/OffscreenCanvas.
- __Refs__:
  - Can I use: <https://caniuse.com/webgpu>
  - Implementation status: <https://github.com/gpuweb/gpuweb/wiki/Implementation-Status>
  - MDN: <https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API>
  - Chrome overview: <https://developer.chrome.com/docs/web-platform/webgpu/overview>

## OffscreenCanvas

- __Status__: Stable in Chromium; Firefox stable (105+); Safari stable (17+).
- __Implication__: Use for off‑main‑thread render/measure; great for diff workers.
- __Refs__:
  - MDN: <https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas>
  - Can I use: <https://caniuse.com/offscreencanvas>

## WebAssembly SIMD

- __Status__: Stable across major engines (Chromium 91+, Firefox 89+,
  Safari 16.4+).
- __Implication__: Enable for text metrics/diff perf; ensure toolchain flags.
- __Refs__:
  - Can I use: <https://caniuse.com/wasm-simd>

## WebAssembly Threads (Shared Memory)

- __Status__: Available with cross‑origin isolation (COOP+COEP). Chrome/Firefox
  support; ensure headers. Safari behavior depends on isolation and platform.
- __Implication__: For multi‑core diff/metrics, serve with COOP/COEP; fall back
  to single‑thread if unavailable.
- __Refs__:
  - web.dev article: <https://web.dev/articles/webassembly-threads>
  - MDN SharedArrayBuffer (security): <https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer>

---

## Recommendations

- __Baseline__: Container Queries, Popover, OffscreenCanvas, Wasm SIMD.
- __Progressive__: Anchor Positioning, View Transitions, Houdini Paint.
- __Experimental/Opt‑in__: WebGPU; Wasm Threads with COOP+COEP.

## Deployment Notes

- __Threads__: Serve with headers
  - Cross-Origin-Opener-Policy: same-origin
  - Cross-Origin-Embedder-Policy: require-corp
- __Fonts__: Host with proper CORS for accurate metrics; avoid layout shifts in calibration.
- __Print__: Verify @page scale at deviceScale=1.0 on calibrated devices.
