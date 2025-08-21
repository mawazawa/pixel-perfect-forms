# Comprehensive Project Structure - August 2025

## Project Root Structure

```text
pixel-perfect-forms/
├── README.md                           # Project overview and quick start
├── CHANGELOG.md                        # Version history and changes
├── LICENSE                            # Project license
├── .gitignore                         # Git ignore patterns
├── .github/                           # GitHub workflows and templates
│   ├── workflows/
│   │   ├── ci.yml                     # Continuous integration
│   │   ├── cd.yml                     # Continuous deployment
│   │   ├── security.yml               # Security scanning
│   │   └── performance.yml            # Performance testing
│   ├── ISSUE_TEMPLATE/                # Issue templates
│   └── PULL_REQUEST_TEMPLATE.md       # PR template
├── docs/                              # Documentation
│   ├── context-engineering/           # Architecture documentation
│   ├── api/                          # API documentation
│   ├── user-guide/                   # User documentation
│   └── deployment/                   # Deployment guides
├── packages/                          # Monorepo packages
│   ├── wasm-core/                    # Rust/WASM core engine
│   ├── web-app/                      # React web application
│   ├── shared-types/                 # TypeScript type definitions
│   └── testing-utils/                # Shared testing utilities
├── scripts/                          # Build and utility scripts
├── tools/                            # Development tools
├── config/                           # Configuration files
└── tests/                            # End-to-end tests
```

## Core WASM Package (packages/wasm-core/)

### Rust/WebAssembly Core Engine

```text
wasm-core/
├── Cargo.toml                         # Rust dependencies and build config
├── Cargo.lock                         # Locked dependency versions
├── build.rs                          # Custom build script
├── wasm-pack-build.js                 # WASM build automation
├── src/
│   ├── lib.rs                        # Main WASM entry point
│   ├── coordinate_system/            # Physical coordinate management
│   │   ├── mod.rs                    # Module definitions
│   │   ├── calibration.rs            # Device calibration logic
│   │   ├── transforms.rs             # Coordinate transformations
│   │   └── validation.rs             # Coordinate validation
│   ├── image_processing/             # Computer vision algorithms
│   │   ├── mod.rs                    # CV module entry
│   │   ├── opencv_integration.rs     # OpenCV.js integration
│   │   ├── edge_detection.rs         # Edge detection algorithms
│   │   ├── template_matching.rs      # Template matching
│   │   ├── ssim_calculation.rs       # SSIM and PSNR metrics
│   │   └── alignment.rs              # Automatic alignment
│   ├── form_processing/              # Form-specific logic
│   │   ├── mod.rs                    # Form processing module
│   │   ├── ca_judicial_forms.rs      # California form specifications
│   │   ├── field_detection.rs        # Form field identification
│   │   ├── validation.rs             # Form validation rules
│   │   └── export.rs                 # Export functionality
│   ├── performance/                  # Performance optimization
│   │   ├── mod.rs                    # Performance module
│   │   ├── simd_utils.rs            # SIMD acceleration utilities
│   │   ├── memory_pool.rs           # Memory management
│   │   └── profiling.rs             # Performance profiling
│   └── utils/                        # Utility functions
│       ├── mod.rs                    # Utils module
│       ├── math.rs                   # Mathematical operations
│       ├── logging.rs                # Logging utilities
│       └── error_handling.rs         # Error types and handling
├── pkg/                              # Generated WASM package
│   ├── pixel_perfect_forms.js        # Generated JS bindings
│   ├── pixel_perfect_forms_bg.wasm   # Compiled WASM binary
│   └── pixel_perfect_forms.d.ts      # TypeScript definitions
├── tests/                            # Rust unit tests
│   ├── integration.rs                # Integration tests
│   ├── coordinate_tests.rs           # Coordinate system tests
│   ├── cv_tests.rs                   # Computer vision tests
│   └── performance_tests.rs          # Performance benchmarks
└── benches/                          # Performance benchmarks
    ├── coordinate_bench.rs           # Coordinate performance
    ├── image_processing_bench.rs     # CV performance
    └── overall_bench.rs              # End-to-end benchmarks
```

### Rust Configuration (Cargo.toml)

```toml
[package]
name = "pixel-perfect-forms"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = "0.3"
console_error_panic_hook = "0.1"
wee_alloc = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.4"

[dependencies.opencv]
version = "0.88"
default-features = false
features = ["opencv-4", "imgproc", "imgcodecs", "features2d"]

[profile.release]
lto = true
opt-level = 3
codegen-units = 1
panic = "abort"

[profile.release.package.opencv]
opt-level = 3

[features]
default = []
simd = []
webgpu = []
```

## Web Application (packages/web-app/)

### React Application Structure

```text
web-app/
├── package.json                       # Node.js dependencies
├── tsconfig.json                      # TypeScript configuration  
├── vite.config.ts                     # Vite build configuration
├── tailwind.config.js                 # Tailwind CSS configuration
├── postcss.config.js                  # PostCSS configuration
├── public/                            # Static assets
│   ├── index.html                     # HTML template
│   ├── manifest.json                  # PWA manifest
│   ├── robots.txt                     # SEO robots file
│   └── assets/                        # Static images and fonts
│       ├── fonts/                     # Form fonts
│       ├── icons/                     # Application icons
│       └── reference-forms/           # Official form images
├── src/
│   ├── index.tsx                      # Application entry point
│   ├── App.tsx                        # Main App component
│   ├── components/                    # React components
│   │   ├── foundation/               # Foundation layer components
│   │   │   ├── CalibrationWizard.tsx # Device calibration UI
│   │   │   ├── CoordinateSystem.tsx  # Coordinate management
│   │   │   ├── ErrorBoundary.tsx     # Error handling
│   │   │   └── PerformanceMonitor.tsx# Performance monitoring
│   │   ├── rendering/                # Rendering layer components
│   │   │   ├── CanvasManager.tsx     # Canvas orchestration
│   │   │   ├── HoudiniPaint.tsx      # CSS Houdini integration
│   │   │   ├── FormRenderer.tsx      # Form rendering engine
│   │   │   ├── OverlayManager.tsx    # Overlay system
│   │   │   └── RulerGrid.tsx         # Rulers and grid
│   │   ├── interaction/              # Interaction layer components
│   │   │   ├── FieldManipulator.tsx  # Field interaction
│   │   │   ├── CVAssistant.tsx       # Computer vision tools
│   │   │   ├── CollaborationHub.tsx  # Real-time collaboration
│   │   │   ├── MeasurementTools.tsx  # Precision measurement
│   │   │   ├── ValidationEngine.tsx  # Form validation
│   │   │   └── WorkflowManager.tsx   # Workflow management
│   │   └── ui/                       # Reusable UI components
│   │       ├── Button.tsx            # Button component
│   │       ├── Modal.tsx             # Modal component
│   │       ├── Tooltip.tsx           # Tooltip component
│   │       └── index.ts              # Component exports
│   ├── hooks/                        # Custom React hooks
│   │   ├── useCalibration.ts         # Calibration hook
│   │   ├── useCoordinates.ts         # Coordinate system hook
│   │   ├── useWASM.ts               # WASM integration hook
│   │   ├── usePerformance.ts         # Performance monitoring
│   │   └── useCollaboration.ts       # Collaboration features
│   ├── services/                     # Business logic services
│   │   ├── wasmService.ts           # WASM integration service
│   │   ├── calibrationService.ts     # Calibration management
│   │   ├── formService.ts           # Form processing service
│   │   ├── exportService.ts         # Export functionality
│   │   └── collaborationService.ts   # Real-time collaboration
│   ├── workers/                      # Web Workers
│   │   ├── wasmWorker.ts            # WASM computation worker
│   │   ├── cvWorker.ts              # Computer vision worker
│   │   └── exportWorker.ts          # Export processing worker
│   ├── houdini/                      # CSS Houdini worklets
│   │   ├── ruler-paint.js           # Ruler painting worklet
│   │   ├── grid-paint.js            # Grid painting worklet
│   │   └── overlay-paint.js         # Overlay painting worklet
│   ├── types/                        # TypeScript type definitions
│   │   ├── forms.ts                 # Form-related types
│   │   ├── coordinates.ts           # Coordinate system types
│   │   ├── wasm.ts                  # WASM interface types
│   │   └── api.ts                   # API response types
│   ├── utils/                        # Utility functions
│   │   ├── coordinates.ts           # Coordinate utilities
│   │   ├── performance.ts           # Performance utilities
│   │   ├── validation.ts            # Validation utilities
│   │   └── export.ts                # Export utilities
│   ├── styles/                       # CSS and styling
│   │   ├── globals.css              # Global styles
│   │   ├── components.css           # Component styles
│   │   ├── print.css                # Print-specific styles
│   │   └── accessibility.css        # Accessibility styles
│   └── __tests__/                    # React component tests
│       ├── components/              # Component tests
│       ├── hooks/                   # Hook tests  
│       ├── services/                # Service tests
│       └── utils/                   # Utility tests
├── cypress/                          # E2E tests
│   ├── e2e/                         # End-to-end test specs
│   ├── fixtures/                    # Test data
│   ├── support/                     # Support files
│   └── plugins/                     # Cypress plugins
└── dist/                            # Built application
```

### Package.json Configuration

```json
{
  "name": "@pixel-perfect/web-app",
  "version": "1.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite --host",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "test": "vitest",
    "test:e2e": "cypress run",
    "lint": "eslint . --ext .ts,.tsx",
    "typecheck": "tsc --noEmit"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "@pixel-perfect/wasm-core": "workspace:*",
    "@pixel-perfect/shared-types": "workspace:*",
    "framer-motion": "^11.0.0",
    "zustand": "^4.4.0",
    "react-hook-form": "^7.45.0",
    "zod": "^3.22.0",
    "@radix-ui/react-dialog": "^1.0.0",
    "@radix-ui/react-tooltip": "^1.0.0",
    "cmdk": "^0.2.0",
    "lucide-react": "^0.280.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "@vitejs/plugin-react": "^4.0.0",
    "vite": "^5.0.0",
    "typescript": "^5.0.0",
    "tailwindcss": "^3.3.0",
    "autoprefixer": "^10.4.0",
    "postcss": "^8.4.0",
    "eslint": "^8.45.0",
    "@typescript-eslint/eslint-plugin": "^6.0.0",
    "prettier": "^3.0.0",
    "vitest": "^1.0.0",
    "@testing-library/react": "^14.0.0",
    "cypress": "^13.0.0",
    "vite-plugin-wasm": "^3.2.0",
    "vite-plugin-top-level-await": "^1.3.0"
  }
}
```

## Shared Types Package (packages/shared-types/)

```text
shared-types/
├── package.json                       # Package configuration
├── tsconfig.json                      # TypeScript configuration
├── src/
│   ├── index.ts                      # Main exports
│   ├── coordinates.ts                # Coordinate system types
│   ├── forms.ts                      # Form-related types  
│   ├── calibration.ts                # Calibration types
│   ├── performance.ts                # Performance monitoring types
│   ├── collaboration.ts              # Collaboration types
│   └── api.ts                        # API interface types
└── dist/                             # Compiled TypeScript
```

## Testing Utilities (packages/testing-utils/)

```text
testing-utils/
├── package.json                       # Package configuration
├── src/
│   ├── index.ts                      # Main exports
│   ├── fixtures/                     # Test data fixtures
│   │   ├── forms.ts                  # Form test data
│   │   ├── coordinates.ts            # Coordinate test data
│   │   └── images.ts                 # Test image data
│   ├── mocks/                        # Mock implementations
│   │   ├── wasmMock.ts              # WASM service mock
│   │   ├── canvasMock.ts            # Canvas API mock
│   │   └── workerMock.ts            # Web Worker mock
│   ├── helpers/                      # Test helper functions
│   │   ├── rendering.ts             # Rendering test helpers
│   │   ├── coordinates.ts           # Coordinate test helpers
│   │   └── performance.ts           # Performance test helpers
│   └── matchers/                     # Custom Jest matchers
│       ├── coordinates.ts           # Coordinate matchers
│       ├── rendering.ts             # Rendering matchers
│       └── performance.ts           # Performance matchers
└── dist/                             # Compiled utilities
```

## Configuration and Scripts

### Root Configuration Files

```text
config/
├── eslint.config.js                  # ESLint configuration
├── prettier.config.js                # Prettier configuration  
├── jest.config.js                    # Jest testing configuration
├── cypress.config.ts                 # Cypress E2E configuration
├── vite.config.base.ts               # Shared Vite configuration
├── tailwind.base.js                  # Base Tailwind configuration
└── tsconfig.base.json                # Base TypeScript configuration
```

### Build and Development Scripts

```text
scripts/
├── build.sh                          # Complete build script
├── dev.sh                            # Development server script
├── test.sh                           # Run all tests
├── deploy.sh                         # Deployment script
├── lint.sh                           # Linting script
├── typecheck.sh                      # Type checking script
├── benchmark.sh                      # Performance benchmarking
└── clean.sh                          # Clean build artifacts
```

### Development Tools

```text
tools/
├── wasm-optimizer/                    # WASM optimization tools
│   ├── optimize.js                   # WASM size optimization
│   └── analyze.js                    # Bundle analysis
├── form-validator/                    # Form validation tools
│   ├── validate-structure.js         # Form structure validation
│   └── validate-measurements.js      # Measurement validation
├── performance-profiler/             # Performance profiling
│   ├── profile.js                    # Performance profiling
│   └── report.js                     # Performance reporting
└── calibration-tester/               # Calibration testing tools
    ├── test-accuracy.js              # Calibration accuracy tests
    └── generate-report.js            # Calibration test reports
```

## Build System Integration

### Monorepo Configuration (package.json)

```json
{
  "name": "pixel-perfect-forms",
  "private": true,
  "workspaces": [
    "packages/*"
  ],
  "scripts": {
    "build": "pnpm -r build",
    "dev": "pnpm -r dev",
    "test": "pnpm -r test",
    "lint": "pnpm -r lint",
    "typecheck": "pnpm -r typecheck"
  },
  "devDependencies": {
    "pnpm": "^8.6.0",
    "nx": "^16.5.0",
    "turbo": "^1.10.0"
  }
}
```

### CI/CD Pipeline Integration

- **GitHub Actions**: Automated testing, building, and deployment
- **Docker**: Containerized development and deployment environments
- **Vercel**: Frontend deployment with edge functions
- **Performance Monitoring**: Automated performance regression testing
- **Security Scanning**: Regular vulnerability assessments

This structure provides a robust foundation for building the pixel-perfect forms application with proper separation of concerns, comprehensive testing, and modern development practices as of August 2025.
