# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a personal website/blog built with Rust using the Actix-web framework. The site serves academic publications and blog posts with content sourced from TOML frontmatter files and Markdown content.

## Development Commands

### Running the Application
```bash
# Run locally with auto-reload
cargo watch -x run

# Run normally
cargo run
```

The application serves on `127.0.0.1:8080` by default.

### CSS/Styling Development
```bash
# Watch for Tailwind CSS changes (run from tailwind/ directory)
cd tailwind
npm run watch-css

# Build CSS for production
npm run build-css-prod
```

### Docker Commands
```bash
# Build and run locally
docker build -t actix-web-app .
docker run -p 8080:8080 actix-web-app

# Build for NAS deployment (linux/amd64)
docker buildx build --platform linux/amd64 -t actix-web-app --output type=docker .
docker save -o actix-web-app.tar actix-web-app
```

## Architecture Overview

### Application Structure
- **main.rs**: Entry point that starts the Actix-web server on port 8080
- **lib.rs**: Core application setup with Tera templating engine and route configuration
- **handlers/**: HTTP request handlers organized by functionality
  - `home_handler.rs`: Homepage with blog post listings
  - `blog_handler.rs`: Blog page functionality
  - `post_handler.rs`: Individual blog post rendering
  - `publications_handler.rs`: Academic publications page

### Content Management
- **Blog Posts**: Located in `posts/` directory with TOML frontmatter (`post_frontmatter.toml`) and Markdown content (`post.md`)
- **Publications**: Located in `publications/` directory with TOML frontmatter files
- **Templates**: HTML templates in `templates/` directory using Tera templating engine
- **Static Assets**: CSS, images, and other static files in `static/` directory

### Key Dependencies
- **actix-web**: Web framework for HTTP server and routing
- **tera**: Templating engine for HTML rendering
- **pulldown-cmark**: Markdown parsing
- **ignore**: File system traversal for content discovery
- **toml**: TOML frontmatter parsing

### Content Discovery Pattern
The application uses a file-walking approach to discover content:
1. Walks through content directories (`posts/`, `publications/`)
2. Finds TOML frontmatter files
3. Parses frontmatter to extract metadata
4. Renders content with associated templates

### Template System
- Uses Tera templating with HTML templates
- Global template instance initialized at startup
- Templates include: `home.html`, `blog.html`, `post.html`, `publications.html`, `base.html`, `navbar.html`
- CSS styling handled by Tailwind CSS compiled from `tailwind/base.css` to `static/css/index.css`