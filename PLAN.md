# FL-100 Pixel-Perfect WASM Forms — Atomic Plan (3 layers × 10)

██████╗ ██╗      █████╗ ███╗   ██╗    ███╗   ███╗██████╗
██╔═══██╗██║     ██╔══██╗████╗  ██║    ████╗ ████║██╔══██╗
 ██║   ██║██║     ███████║██╔██╗ ██║    ██╔████╔██║██████╔╝
 ██║   ██║██║     ██╔══██║██║╚██╗██║    ██║╚██╔╝██║██╔══██╗
 ╚██████╔╝███████╗██║  ██║██║ ╚████║    ██║ ╚═╝ ██║██║  ██║
  ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝  ╚═══╝    ╚═╝     ╚═╝╚═╝  ╚═╝

 PLAN.md

- Date: 2025-08-21
- Owner: mawazawa
- Goal: Rust+WASM app recreates CA Judicial Council forms as screen-first, print-accurate,
  verifiable clones (no PDFs). Calibrated units, per-page overlay, CV-assisted proofing.

## Research (2025) summary

- Houdini Paint: Chromium OK; fallback present.
- Anchor+Popover: Chromium OK; fallbacks.
- View Transitions: nice-to-have.
- Container Queries: broadly OK.
- WebGPU: optional accel, feature-detect.
- OffscreenCanvas: OK in Workers.
- Wasm SIMD/Threads: enable for CV.

## L1 overview (10)

1) Fix build + proofing UI
2) Single scroll doc (3 pages)
3) Calibration (deviceScale)
4) Units migration (mm/pt)
5) Per-page overlays + xform
6) Rulers + grid + snap
7) Fonts + metrics fidelity
8) Diff worker + auto-align
9) Print CSS + export
10) DevX + Rust/WASM + CI

---

## L1-1. Fix build + unify proofing UI

- L2-1. KeyboardShortcuts prop fix
- L3-1: Capture TS errors
- L3-2: Compare used vs typed
- L3-3: Finalize API (visible,onClose)
- L3-4: Default props
- L3-5: Remove dead code
- L3-6: Compile clean
- L3-7: Unit tests
- L3-8: Storybook
- L3-9: A11y (focus, ESC)
- L3-10: Docs page
- L2-2. ProofModeControls component
- L3-1: Define controls (toggle/opacity/tx/ty/scale/rot)
- L3-2: Stateless props
- L3-3: Fine nudge keys
- L3-4: Sliders w/ snapping
- L3-5: Reset per page
- L3-6: Persist state
- L3-7: Tests
- L3-8: Storybook
- L3-9: A11y labels
- L3-10: Perf audit
- L2-3. Command palette
- L3-1: Command IDs
- L3-2: Keymap register
- L3-3: Palette UI
- L3-4: Scope to page
- L3-5: Undoable ops
- L3-6: Conflict resolve
- L3-7: Dispatch tests
- L3-8: Telemetry
- L3-9: Docs
- L3-10: A11y
- L2-4. Global UI store
- L3-1: Types (deviceScale,pages,fields,settings)
- L3-2: Selectors
- L3-3: Persist
- L3-4: Migration ver
- L3-5: Devtools
- L3-6: Reducer tests
- L3-7: Memo perf
- L3-8: Error boundaries
- L3-9: Event bus
- L3-10: Schema docs
- L2-5. Popover+Anchor overlays
- L3-1: Audit overlays
- L3-2: Use popover/anchor
- L3-3: Fallback JS pos
- L3-4: Focus mgmt
- L3-5: Close behaviors
- L3-6: Kbd/mouse tests
- L3-7: RTL
- L3-8: Layout thrash low
- L3-9: Docs patterns
- L3-10: A11y
- L2-6. Error toasts/logging
- L3-1: Logger
- L3-2: Toast bus
- L3-3: Worker bridge
- L3-4: Source maps
- L3-5: Persist last
- L3-6: Screenshot hotkey
- L3-7: Redact PII
- L3-8: Tests
- L3-9: Docs
- L2-7. Minimal telemetry
  - L3-1: Consent gate
- L3-2: Event schema
- L3-3: Batch sender
- L3-4: Sample rates
- L3-5: Perf marks
- L3-6: Toggle UI
- L3-7: Tests
- L3-8: Docs
- L3-9: Export raw
- L3-10: Privacy review
- L2-8. Theming baseline
  - L3-1: Tokens
- L3-2: Light/dark
- L3-3: High-contrast
- L3-4: Density
- L3-5: Motion reduce
- L3-6: Type scale
- L3-7: Spacing
- L3-8: Docs
- L3-9: Visual QA
- L3-10: Icons
- L2-9. CI checks
- L3-1: TS strict
- L3-2: ESLint
- L3-3: Prettier
- L3-4: Unit tests
- L3-5: Storybook build
- L3-6: Lighthouse
- L3-7: Bundle guard
- L3-8: E2E smoke
- L3-9: Artifacts
- L3-10: PR gates
- L2-10. Docs + demos
- L3-1: README
- L3-2: GIFs
- L3-3: Key map
- L3-4: Troubleshoot
- L3-5: FAQ
- L3-6: Contrib
- L3-7: Changelog
- L3-8: Versioning
- L3-9: Roadmap
- L3-10: Support

---

## L1-2. Single scrollable document (3 pages)

- L2-1. Page container spec
- L3-1: Letter consts
- L3-2: mm/pt conv
- L3-3: Bleed/margins
- L3-4: Page edges
- L3-5: Shadows
- L3-6: Bg color
- L3-7: Print-safe
- L3-8: DPI tests
- L3-9: Storybook
- L3-10: Docs
- L2-2. Scroll/navigation
- L3-1: Natural stack
- L3-2: Sticky header
- L3-3: Mini-map
- L3-4: Jump to page
- L3-5: PgUp/PgDn
- L3-6: Snap edges
- L3-7: Restore scroll
- L3-8: E2E
- L3-9: Perf
- L3-10: Docs
- L2-3. Edges + shadows
- L3-1: Crisp edges
- L3-2: DPI-tuned shadow
- L3-3: Theme-aware
- L3-4: Focus outline
- L3-5: Print remove
- L3-6: Visual QA
- L3-7: Storybook
- L3-8: Snapshots
- L3-9: Tokens
- L3-10: Docs
- L2-4. Pages state model
- L3-1: {id,ref,transform}
- L3-2: Selectors
- L3-3: Persist
- L3-4: Migrate
- L3-5: Tests
- L3-6: Perf
- L3-7: Errors
- L3-8: Docs
- L3-9: Devtools
- L3-10: Seed
- L2-5. Virtualization
- L3-1: IO observers
- L3-2: Pre-render offscreen
- L3-3: ImageBitmap cache
- L3-4: Canvas pool
- L3-5: Pause diff hidden
- L3-6: Memory caps
- L3-7: Tests
- L3-8: Perf budget
- L3-9: Telemetry
- L3-10: Docs
- L2-6. Selection model
- L3-1: Focus ring
- L3-2: Marquee select
- L3-3: Kbd move
- L3-4: Align/distrib
- L3-5: Snap edges
- L3-6: Delete/dup
- L3-7: Z-order
- L3-8: Tests
- L3-9: A11y
- L3-10: Docs
- L2-7. Inspector panel
- L3-1: Anchor+Popover
- L3-2: Position mm
- L3-3: Size mm
- L3-4: Font controls
- L3-5: Letter/line
- L3-6: Values
- L3-7: Undo/redo
- L3-8: Tests
- L3-9: Storybook
- L3-10: Docs
- L2-8. Undo/redo
- L3-1: Command pattern
- L3-2: History cap
- L3-3: Grouping
- L3-4: Cross-page
- L3-5: Persist snaps
- L3-6: Kbd binds
- L3-7: Tests
- L3-8: Perf
- L3-9: UX afford
- L3-10: Docs
- L2-9. Autosave + versions
- L3-1: Interval save
- L3-2: Version stamp
- L3-3: Restore flow
- L3-4: JSON import/export
- L3-5: Quota guard
- L3-6: Telemetry
- L3-7: Tests
- L3-8: Docs
- L3-9: Conflict alert
- L3-10: Copy
- L2-10. E2E flows
- L3-1: Load ref
- L3-2: Calibrate
- L3-3: Place fields
- L3-4: Align overlay
- L3-5: Diff check
- L3-6: Print preview
- L3-7: Save
- L3-8: Reload
- L3-9: Export
- L3-10: CI smoke

---

## Phases

- Phase A: L1-1, L1-2, L1-3, L1-4
- Phase B: L1-5, L1-6
- Phase C: L1-7, L1-8
- Phase D: L1-9, L1-10

## Acceptance

- Screen vs physical <0.25mm
- Diff score under target
- Print 1:1 within <0.25mm
- Components <200 LOC; CI green