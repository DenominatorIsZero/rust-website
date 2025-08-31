# Architecture Documentation

Technical overview of my personal website's architecture, the decisions I made, and how everything fits together.

## Table of Contents

1. [Project Overview & Design Philosophy](#project-overview--design-philosophy)
2. [Core Architecture](#core-architecture)
3. [Content Management System](#content-management-system)
4. [AI Demo Integration Architecture](#ai-demo-integration-architecture)
5. [Technology Stack & Dependencies](#technology-stack--dependencies)
6. [Docker Deployment Architecture](#docker-deployment-architecture)
7. [Development Workflow](#development-workflow)

---

## Project Overview & Design Philosophy

### My Learning Platform

I built this website as my personal learning platform for exploring modern web technologies with Rust. Coming from academia to working in AI, I wanted to experiment with:

- **New Technologies**: Trying out Rust for web development and WebAssembly for AI demos
- **Automated Content**: Making it easy to publish papers and blog posts
- **Browser AI**: Running AI models directly in the browser without APIs

### Core Principles

**1. Content-Centric Design**

- I wanted to easily showcase my academic publications and write blog posts
- TOML frontmatter + Markdown makes it simple to add new content
- The file system automatically discovers new posts and papers

**3. Modern Web Technologies**

- Learning Rust web development with Actix-web
- WebAssembly lets me run AI models in browsers
- Tailwind CSS for responsive design (I'm not a designer!)

---

## Core Architecture

### Server Architecture

**Framework**: Actix-web with middleware and service routing

```rust
// Demo handler serves individual demo pages
#[get("/demos/{demo_name}")]
pub async fn demo(tmpl: web::Data<tera::Tera>, demo_name: web::Path<String>) -> impl Responder {
    let mut context = tera::Context::new();

    let demo_context = match DemoContext::new(&demo_name) {
        Ok(s) => s,
        Err(e) => {
            return HttpResponse::NotFound()
                .content_type("text/html")
                .body("<p>Could not find demo - sorry!</p>");
        }
    };

    context.insert("demo_context", &demo_context);

    match tmpl.render("demo.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => HttpResponse::NotFound().body("<p>Could not render demo - sorry!</p>")
    }
}
```

I picked Actix-web because it's fast and Rust catches the memory bugs I'd probably write in C++. The static file serving works well for WASM demos and CSS.

### Template System Architecture

**Template Engine**: Tera with global template initialization

```rust
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}
```

**Template Hierarchy:**

```
templates/
├── base.html          # Base layout with navigation, footer, analytics
├── home.html          # Homepage with blog post listings
├── blog.html          # Blog overview page
├── post.html          # Individual blog post rendering
├── publications.html  # Academic publications listing
├── demos.html         # AI demonstrations overview
├── demo.html          # Individual demo embedding
├── privacy.html       # Privacy policy and GDPR information
└── navbar.html        # Navigation component
```

The template system is straightforward - base template handles layout, shared navbar means I only update it once. Templates auto-escape HTML so I don't worry about XSS.

### Request Handler Organization

**Handler Structure** (`src/handlers/`):

```
handlers/
├── mod.rs                  # Handler module exports
├── home_handler.rs         # Homepage with content discovery
├── blog_handler.rs         # Blog overview functionality
├── post_handler.rs         # Individual post rendering and parsing
├── publications_handler.rs # Academic publications display
├── demo_handler.rs         # AI demo integration and serving
└── privacy_handler.rs      # Privacy policy rendering
```

**Handler Responsibilities:**

- **Content Discovery**: File system traversal for posts and publications
- **Template Rendering**: Context preparation and template invocation
- **Response Formatting**: Proper HTTP headers and content types

---

## Content Management System

### File-Based Content Architecture

**Content Discovery Pattern**: Automated file system traversal

```rust
fn find_all_publications() -> Result<Vec<Publication>, std::io::Error> {
    let mut t = ignore::types::TypesBuilder::new();
    t.add_defaults();
    let toml = t.select("toml").build()?;

    let file_walker = WalkBuilder::new("./publications").types(toml).build();
    let mut publications = Vec::new();

    for entry in file_walker {
        if let Ok(file) = entry {
            if file.path().is_file() {
                let content = fs::read_to_string(file.path())?;
                let publication: Publication = toml::from_str(&content)?;
                publications.push(publication);
            }
        }
    }

    publications.sort_by(|a, b| b.date.cmp(&a.date));
    Ok(publications)
}
```

**Directory Structure:**

```
content/
├── posts/
│   └── post-name/
│       ├── post_frontmatter.toml    # Metadata (title, date, tags)
│       └── post.md                  # Markdown content
├── publications/
│   └── publication.toml             # Publication metadata
└── demos/
    └── demo-name/
        └── demo_frontmatter.toml    # Demo configuration
```

### Content Processing Pipeline

**TOML Frontmatter Processing:**

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct Demo {
    pub title: String,
    pub description: String,
    pub instructions: String,
    pub status: String,
    pub date: String,
    pub wasm_path: String,
    pub repository: String,
    pub thumbnail: String,
    pub demo_type: String,
    pub tech_stack: Vec<String>,
    pub keywords: Vec<String>,
    pub model_size: String,
    pub browser_requirements: String,
    pub performance_notes: String,
}
```

**Content Rendering Plans:**

- **Blog Posts**: Will render from Markdown or Typst files (still figuring this out)
- **Publications**: Already working with TOML metadata
- **AI Demos**: TOML frontmatter with embedded WebAssembly content

**Current Content Flow:**

1. **Discovery**: File system walker finds TOML frontmatter files
2. **Parsing**: TOML deserialization into typed structures
3. **Validation**: Content validation and error handling
4. **Template Rendering**: Context preparation and HTML generation

I like that everything is just files I can version control and search through. TOML frontmatter catches typos at compile time, and the file walker finds content automatically.

## AI Demo Integration Architecture

### WebAssembly Demo Integration

**Demo Serving Strategy**: Iframe embedding with responsive design

The demo handler works the same way as other content: it reads TOML frontmatter files from the `demos/` directory and renders them with templates. Each demo includes metadata about the WebAssembly files, model information, and display settings.

**Static Asset Organization:**

```
static/
├── wasm/
│   └── demo-name/
│       ├── interactive.wasm        # Compiled WebAssembly binary
│       ├── interactive.js          # JavaScript bindings
│       └── index.html              # Demo HTML wrapper
├── models/
│   └── demo-name/
│       ├── model.safetensors       # ML model weights
│       └── model.toml              # Model metadata
└── img/
    └── demos/
        └── demo-name.png           # Demo preview images
```

### AI Demo Architecture Pattern

**Standardized Demo Structure**:

- **Training Binary**: Native Rust for model training
- **Interactive Binary**: Bevy + Candle compiled to WASM
- **Shared Library**: Common model definitions and utilities
- **Model Assets**: .safetensors format for security and performance

Each demo is self-contained with all its assets. SafeTensors format means no worrying about malicious model files. Iframes work fine for embedding.

### Browser AI Inference Architecture

**Technology Stack for Demos:**

- **Bevy Framework**: ECS-based UI and rendering system
- **Candle**: Rust-native AI inference library
- **WASM Target**: `wasm32-unknown-unknown` compilation
- **WebAssembly**: Browser-native execution environment

**Performance Considerations:**

- WASM binary optimization with release profiles
- Progressive loading with user feedback

---

## Technology Stack & Dependencies

### Backend Dependencies

**Core Framework:**

```toml
[dependencies]
actix-web = "4.11.0"      # Web framework and HTTP server
tera = "1"                # Template engine
actix-files = "0.6.1"     # Static file serving
```

**Content Processing:**

```toml
pulldown-cmark = { version = "0.13.0", default-features = false, features = ["html"]}
toml = "0.9.2"            # TOML frontmatter parsing
serde = { version = "1.0.219", features = ["derive"] }
```

**File System & Discovery:**

```toml
ignore = "0.4.23"         # Git-aware file traversal
```

**Development & Logging:**

```toml
env_logger = "0.11.8"     # Structured logging
lazy_static = "1.4.0"     # Global template initialization
```

### Frontend Technologies

**CSS Framework**: Tailwind CSS with custom build pipeline

```bash
# Tailwind build process
cd tailwind
npm run build-css-prod    # Production CSS compilation
npm run watch-css         # Development with hot reload
```

**JavaScript**: Vanilla JavaScript for PostHog analytics (with basic GDPR compliance) and progressive enhancement

**Styling Architecture:**

- **Base Styles**: `tailwind/base.css` with custom components
- **Build Output**: `static/css/index.css` for production serving
- **Design System**: Consistent color palette and typography
- **Responsive Design**: Mobile-first with breakpoint utilities

---

## Docker Deployment Architecture

### Multi-Stage Build Process

**Dockerfile Architecture:**

```dockerfile
# Stage 1: Rust compilation
FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Stage 2: Runtime image
FROM ubuntu:22.04
RUN apt-get update && apt-get install -y libssl-dev
COPY --from=builder /usr/src/app/target/release/personal-website /usr/local/bin/
COPY ./posts /posts
COPY ./demos /demos
COPY ./publications /publications
COPY ./static /static
COPY ./templates /templates
```

### Asset Management

**Static Asset Strategy:**

- Complete asset copying into Docker image
- No runtime dependency on external file systems
- WASM demos and models included in deployment
- CSS compilation happens before Docker build

**Volume Strategy:**

- No persistent volumes required
- Stateless deployment suitable for containers
- Content updates require image rebuild

### Production Deployment Workflow

**Build Process:**

```bash
# Local development
cargo watch -x run

# Production build
docker buildx build --platform linux/amd64 -t rust-website --output type=docker .
docker save -o rust-website.tar rust-website
```

---

## Development Workflow

### CSS Development Pipeline

**Tailwind Integration:**

```bash
# Development workflow
cd tailwind
npm install                # Install Tailwind dependencies
npm run watch-css          # Watch for changes and recompile
cargo watch -x run         # Rust server with hot reload
```

**CSS Build Process:**

1. **Source**: `tailwind/base.css` with custom styles and Tailwind directives
2. **Compilation**: Tailwind CSS processes and generates utilities
3. **Output**: `static/css/index.css` served by Actix-web static files
4. **Optimization**: Production builds include purging and minification

### Content Development

**Blog Post Workflow:**

1. Create directory: `posts/post-slug/`
2. Add frontmatter: `post_frontmatter.toml` with metadata
3. Write content: `post.md` with Markdown
4. Server automatically discovers and renders new content

**Publication Workflow:**

1. Add publication metadata: `publications/title.toml`
2. Include PDF links and academic details
3. Automatic rendering in publications list

### Quality Assurance

**Code Quality Tools** (recommended):

```bash
cargo fmt              # Code formatting
cargo clippy           # Linting and suggestions
cargo test             # Test suite execution
cargo audit            # Security vulnerability scanning
```

**Performance Monitoring:**

- Request logging with Actix middleware
- Static asset caching headers
- WASM bundle size optimization
- Page load performance tracking

---

## Architecture Benefits

### For My Development Process

- 🦀 **Rust Learning**: Great way to learn systems programming concepts
- 🎯 **Catches Mistakes**: Compiler finds issues before users see them
- 🔄 **Fast Iteration**: cargo-watch rebuilds automatically when I make changes
- 📁 **Version Everything**: All my content lives in Git alongside the code

### For Managing My Content

- 📝 **Easy Writing**: I can write posts in Markdown
- 🏷️ **Organized Metadata**: TOML frontmatter keeps everything structured
- 🔍 **Automatic Publishing**: Just add a file and the site finds it
- ⚡ **Fast Loading**: Templates cache so pages load quickly

### For My AI Experiments

- 🌐 **No API Needed**: AI models run directly in users' browsers
- 📦 **Fast Execution**: WebAssembly gives near-native performance
- 🎨 **Flexible Display**: Iframes let me embed demos anywhere
- 🔄 **Reusable Pattern**: Once I figured it out, I can apply it to new demos

This architecture works well for my needs. It's a privacy-compliant personal website where I can showcase my work and experiment with AI in browsers. It's been a great learning experience building it with Rust.
