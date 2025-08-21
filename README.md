# Pixel-Perfect Forms

A Rust + WebAssembly application that recreates California Judicial Council forms as pixel-perfect, verifiable clones without the limitations of PDFs.

## Overview

This project creates screen-first, print-accurate recreations of legal forms using:
- **Rust/WebAssembly Core**: SIMD-accelerated image processing and coordinate management
- **Modern Web APIs**: CSS Houdini, WebGPU compute shaders, OffscreenCanvas
- **Computer Vision**: OpenCV.js for automatic alignment and quality verification
- **Physical Unit System**: Calibrated coordinate system for sub-millimeter accuracy

## Key Features

- ✅ **Sub-millimeter Precision**: ±0.25mm positioning accuracy
- ✅ **Device Calibration**: User-calibrated coordinate system for accurate measurements  
- ✅ **Computer Vision Alignment**: Automated form alignment using OpenCV.js
- ✅ **Real-time Collaboration**: Multi-user editing with conflict resolution
- ✅ **Legal Compliance**: Full California Judicial Council form specifications
- ✅ **Accessibility**: WCAG 2.1 AAA compliance with screen reader support
- ✅ **Cross-platform**: Works on desktop and mobile with touch optimization

## Technology Stack (August 2025)

### Core Technologies
- **Rust + WebAssembly**: Core processing engine with SIMD optimization
- **TypeScript + React 18**: Modern web application framework
- **CSS Houdini Paint API**: Custom rendering with hardware acceleration
- **WebGPU**: Optional compute shader acceleration for CV operations
- **OpenCV.js**: Computer vision algorithms compiled to WebAssembly

### Modern Web APIs
- Container Queries for responsive design
- CSS Anchor Positioning for precise overlays
- Popover API for accessible modals
- OffscreenCanvas for background processing
- Web Workers for parallel computation

## Architecture

The application is structured in three layers:

### 1. Foundation Layer
- Physical coordinate system with device calibration
- WebAssembly runtime with SIMD optimization
- Performance monitoring and error handling
- Configuration management and testing infrastructure

### 2. Rendering Layer
- Multi-layer canvas management system
- CSS Houdini Paint integration for rulers and grids
- Form rendering engine with pixel-perfect typography
- Advanced overlay system with transformation controls

### 3. Interaction Layer
- Sophisticated form field manipulation
- Computer vision assisted alignment
- Real-time collaboration capabilities
- Comprehensive accessibility features

## Documentation

Comprehensive documentation is available in the [`docs/context-engineering/`](docs/context-engineering/) directory:

- [Technology Assessment 2025](docs/context-engineering/technology-assessment-2025.md)
- [Architectural Specifications](docs/context-engineering/architectural-specifications/)
- [Research Findings](docs/context-engineering/research-findings/)
- [Technical Specifications](docs/context-engineering/technical-specifications/)

## Development

This project uses a monorepo structure with the following packages:

- `packages/wasm-core/`: Rust/WebAssembly core engine
- `packages/web-app/`: React web application
- `packages/shared-types/`: TypeScript type definitions
- `packages/testing-utils/`: Shared testing utilities

## Legal Compliance

This project strictly adheres to California Judicial Council form specifications and legal requirements:

- Forms validated against official PDF specifications
- Compliance with rule 1.44 (computer-generated duplicates)
- Audit trails for all form modifications
- Accessibility compliance for government forms

## License

[License information to be determined]

## Contributing

[Contributing guidelines to be added]

---

**Built with modern web technologies for the 75 million self-represented litigants navigating the legal system.**