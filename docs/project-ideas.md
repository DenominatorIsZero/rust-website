# AI Demo Project Ideas

This document outlines potential interactive AI demonstrations for the website. The primary goals are gaining practical experience with AI techniques, creating engaging educational content, and showcasing browser-based inference capabilities.

## Technical Requirements

- Client-side inference (no external API dependencies)
- Built on existing Rust architecture
- Integration with current actix-web/tera setup
- WASM compilation for browser deployment

## Architecture Overview

Current site infrastructure:
- actix-web web server
- tera templating engine
- Static file serving
- TOML frontmatter content management (posts, publications)

Proposed extensions:
- `demos/` directory following existing content patterns
- WASM asset compilation and serving
- Model file hosting and caching
- Browser-based AI inference pipeline

## Project Categories

### Foundation Projects

**Handwritten Digit Recognition**
Canvas-based digit drawing with real-time CNN inference. Validates the complete WASM/candle pipeline while remaining technically manageable. Small model size (~500KB) ensures good user experience.

**Audio Transcription (Whisper)**
Leverages existing HuggingFace candle WASM implementations. Demonstrates practical AI applications with audio file upload and microphone input support.

**Text Generation**
Small language model deployment (Phi/TinyLlama) for interactive text completion. Explores browser LLM capabilities and privacy-focused inference.

### Research Integration

**Biomedical Signal Analysis**
Interactive ECG/EMG signal visualization and analysis. Direct connection to magnetocardiography research. Potential for educational content explaining signal processing concepts.

**Motion Analysis Visualization**
Webcam-based pose estimation with biomechanical analysis components. Builds on recent motion tracking research publications. Demonstrates real-world application of academic work.

### Advanced Visualizations

**Bevy-Powered Interactive Simulations**
- Neural network flocking behaviors (boids with learning agents)
- 3D neural network visualization playground
- Signal processing in interactive 3D environments
- Physics-based reinforcement learning sandbox

**Generative Content**
- Procedural world generation with AI-driven terrain/weather/ecosystems
- Real-time music analysis with synchronized visual effects
- Neural style transfer for artistic image transformation

## Standardized Project Architecture

**Repository Structure**
Each AI demo will be implemented as a separate repository with a consistent Cargo workspace structure:

```
demo-project-name/
├── Cargo.toml                 # Workspace root
├── training/
│   ├── Cargo.toml            # Native training binary
│   ├── src/main.rs           # Model training implementation
│   └── datasets/             # Training data and scripts
├── interactive/
│   ├── Cargo.toml            # WASM demo binary  
│   ├── src/main.rs           # Bevy + Candle inference
│   └── assets/               # Bevy-specific assets
├── shared/
│   ├── Cargo.toml            # Shared library crate
│   └── src/                  # Common model definitions and utilities
└── models/                   # Generated .safetensors files
```

**Technology Stack Standardization**
- **Interactive demos**: Bevy framework compiled to WASM
- **AI inference**: Candle for all model operations
- **Model format**: .safetensors for consistency and security
- **Build process**: wasm-pack for WASM compilation

**Integration with Main Site**
- WASM files served from `static/wasm/project-name/`
- Model files served from `static/models/project-name/`
- Demos embedded in iframes for flexible blog integration
- Consistent `/demos/project-name` URL structure

**Benefits of This Approach**
- Single development pipeline to master and optimize
- Code reuse across projects for common functionality
- Complete reproducibility with training and inference code together
- Easy embedding of interactive elements in blog posts
- Simplified deployment and asset management

## Technical Implementation

**Model Constraints**
Target model sizes under 50MB for reasonable download times. WASM bundle sizes expected to be 2-5MB per demo after compression.

**Performance Considerations**
WASM inference performance requires optimization focus. Plan for profiling and iterative improvements. Bevy provides consistent performance characteristics across projects.

**Browser Compatibility**
Progressive enhancement approach with graceful degradation. Mobile device support with appropriate performance expectations. Consistent loading patterns across all demos.

## Content Development

Each demonstration should include:
- Technical explanation of underlying AI methods
- Implementation details and architecture decisions
- Performance analysis and optimization notes
- Connections to practical applications where relevant

Potential blog post topics:
- Rust ecosystem for browser AI development
- Performance comparisons: local inference vs API services
- Academic research translation to interactive demos
- Technical deep-dives on specific implementations

## Implementation Strategy

Starting with digit recognition provides pipeline validation with manageable complexity. Follow with adaptation of existing candle WASM examples before developing custom implementations.

Priority is shipping functional demonstrations over extensive feature sets. Iterative development with user feedback incorporation.

## Open Technical Questions

- Candle WASM performance characteristics vs native implementation
- Optimal model size thresholds for various connection speeds
- Workspace organization for WASM sub-projects
- Model loading and browser caching strategies
- Integration patterns with existing site architecture

This document will be updated as implementation progresses and technical questions are resolved.