# Rust Personal Website

A personal website built with Rust and Actix-web, featuring interactive AI demonstrations compiled to WebAssembly. This project serves as a learning platform for exploring modern web technologies, AI inference in browsers, and Rust-based web development.

**Live Site**: [erik-engelhardt.com](https://erik-engelhardt.com)  
**Repository**: [github.com/DenominatorIsZero/rust-website](https://github.com/DenominatorIsZero/rust-website)

## Overview

This is a personal learning project where I experiment with new technologies. I'm primarily focused on:

- **Rust web development** with Actix-web
- **WebAssembly AI demos** using the Candle framework
- **Template-driven content** for publications and blog posts

> **Note**: I'm not a web developer by profession—what you see here represents my best effort at learning these technologies. The code is open source in case anyone needs inspiration for building a personal website with Rust.

## Features

### 🤖 Interactive AI Demos

- Run AI models directly in your browser using WebAssembly
- Built with Bevy and compiled to WASM
- No external APIs needed, everything runs locally
- Check out the demos at `/demos/`

### 📚 Academic Publications

- Lists my publications automatically from TOML files
- Clean presentation of research work

### 🔒 Privacy-First Analytics

- GDPR-compliant analytics with PostHog
- Full consent management built in
- Privacy settings are always accessible
- Uses EU servers for data residency

### ⚙️ Template-Driven Architecture

- Builds content automatically from TOML + Markdown files
- Uses Tera templating for consistent layouts
- Discovers content by walking the file system
- Hot-reload during development

## Technology Stack

**Backend**: Rust (Actix-web, Tera, pulldown-cmark)  
**Frontend**: HTML templates, Tailwind CSS, Vanilla JavaScript  
**AI Demos**: Rust (Bevy) compiled to WebAssembly via Candle  
**Deployment**: Docker on personal infrastructure  
**Analytics**: PostHog (EU instance) with GDPR compliance

## Quick Start

### Prerequisites

- Rust 1.70+
- Node.js (for Tailwind CSS)
- Docker (for deployment)

### Development

This project uses [just](https://github.com/casey/just) for command management:

```bash
# Clone the repository
git clone https://github.com/DenominatorIsZero/rust-website.git
cd rust-website

# Install development tools and dependencies
just setup

# Start development server with hot reload
just dev

# Visit http://127.0.0.1:8080
```

### CSS Development

```bash
just css-watch     # Watch and rebuild CSS
just css-build     # Build production CSS
```

### Docker Deployment

```bash
# Build and run locally
just docker-build
docker run -p 8080:8080 rust-website

# Build and package for deployment
just docker-deploy
```

## Project Structure

```
├── src/
│   ├── handlers/          # Request handlers (home, blog, publications, etc.)
│   ├── lib.rs             # Application setup and routing
│   └── main.rs            # Server entry point
├── templates/             # Tera HTML templates
├── static/                # CSS, images, and WASM files
│   ├── css/              # Compiled Tailwind CSS
│   ├── wasm/             # AI demo WebAssembly binaries
│   └── img/              # Images and assets
├── posts/                 # Blog posts (TOML frontmatter + Markdown)
├── publications/          # Academic publications (TOML files)
├── demos/                 # AI demo metadata and configurations
├── tailwind/              # Tailwind CSS source and build config
└── docs/                  # Project documentation and workflow templates
    └── projects/          # AI-assisted development workflow
```

## AI Demo Architecture

Each AI demo follows the same basic structure:

- **Training**: Native Rust binary for training models
- **Interactive**: Bevy + Candle app compiled to WASM
- **Integration**: Files served from `/static/wasm/demo-name/`
- **Models**: Uses .safetensors format for security and performance

For detailed architecture documentation, see `docs/architecture.md` (coming soon).

## Content Management

### Blog Posts

```
posts/post-name/
├── post_frontmatter.toml  # Metadata (title, date, tags)
└── post.md                # Content in Markdown
```

### Publications

```
publications/
└── publication.toml       # Publication metadata and details
```

The application automatically discovers and renders content from these directories.

## Development Workflow

This project follows a systematic AI-assisted development approach:

1. **Specification Phase**: Define requirements and success criteria
2. **Planning Phase**: Break down into actionable tasks
3. **Implementation Phase**: Execute with frequent commits

Templates and documentation for this workflow are available in `docs/projects/template/`.

## Security

This project follows security best practices for web applications:

### Security Features

- **Dependency Scanning**: Regular vulnerability audits with `cargo audit`
- **Template Security**: Tera templating with HTML auto-escaping enabled
- **Static Assets**: Secure serving with appropriate caching headers
- **Model Safety**: Uses .safetensors format for AI models to prevent arbitrary code execution
- **GDPR Compliance**: Privacy-first analytics with explicit user consent

### Security Practices

- Dependencies are regularly updated and audited for vulnerabilities
- Input validation and sanitization on all user-facing endpoints
- Error handling that prevents information disclosure
- Secure static file serving configuration

### Reporting Vulnerabilities

If you discover security issues, please report them responsibly:

- **Contact**: [Erik Engelhardt on LinkedIn](https://www.linkedin.com/in/erik-engelhardt-65b1091a7/)

## Contributing

This is primarily a personal learning project, but I welcome feedback and suggestions!

- **Bug Reports**: Feel free to open an issue
- **General Questions**: Happy to discuss the technical approach

For detailed contributing guidelines, see `CONTRIBUTING.md`.

## Background

I recently transitioned from academia to working in the AI sector. This website showcases both my academic publications and my current explorations in AI technology, particularly browser-based inference and WebAssembly applications.

The site represents my journey learning modern web technologies—if you're also exploring Rust for web development or AI in browsers, I hope this code provides useful inspiration!

## Contact

**Erik Engelhardt**  
[LinkedIn](https://www.linkedin.com/in/erik-engelhardt-65b1091a7/)

## License

MIT License - see `LICENSE` file for details.

---

_This website is built with Rust and deployed at [erik-engelhardt.com](https://erik-engelhardt.com)_
