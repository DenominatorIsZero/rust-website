# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a personal website/blog built with Rust using the Actix-web framework. The site serves academic publications and blog posts with content sourced from TOML frontmatter files and Markdown content. The site includes GDPR-compliant analytics tracking using PostHog with a comprehensive consent management system.

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
  - `privacy_handler.rs`: Privacy policy page

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
- Templates include: `home.html`, `blog.html`, `post.html`, `publications.html`, `privacy.html`, `base.html`, `navbar.html`
- CSS styling handled by Tailwind CSS compiled from `tailwind/base.css` to `static/css/index.css`

## Analytics & Privacy Implementation

### PostHog Analytics Integration
- **Service**: PostHog EU instance (`eu.i.posthog.com`) for GDPR compliance
- **Configuration**: Privacy-focused with `person_profiles: 'identified_only'`
- **Location**: JavaScript implementation in `base.html` template
- **API Key**: Public key hardcoded in template (safe for frontend use)

### GDPR Consent Management System
The site implements a comprehensive GDPR-compliant consent system:

#### Consent Banner (`base.html:152-170`)
- Appears on first visit if no consent stored
- Non-intrusive bottom banner with clear Accept/Reject buttons
- Links to privacy policy for detailed information
- Uses green theme for Accept button to match site design

#### Privacy Settings Modal (`base.html:172-203`)
- Always accessible via "Privacy Settings" link in footer
- Shows current consent status (Enabled/Disabled)
- Toggle button to easily change preferences
- Changes take effect immediately
- **Critical for GDPR**: Allows easy consent withdrawal

#### Consent State Management (JavaScript)
- **Storage**: Uses localStorage for consent preferences
- **Keys**: `analytics-consent` (boolean) and `analytics-consent-date` (ISO timestamp)
- **Expiration**: Automatically expires after 12 months, re-prompting user
- **PostHog Integration**: Only initializes PostHog with explicit user consent

#### Key Functions in `base.html`:
- `hasConsent()`: Checks current consent status and expiration
- `setConsent(boolean)`: Sets consent preference and initializes/stops PostHog
- `showPrivacyModal()` / `hidePrivacyModal()`: Modal management
- `toggleConsent()`: Switches consent state with immediate effect
- `updatePrivacyModalContent()`: Updates modal to reflect current state

### Privacy Policy (`templates/privacy.html`)
- Comprehensive GDPR-compliant privacy policy
- Accurate description of PostHog usage and data collection
- Clear instructions for consent withdrawal via Privacy Settings
- Contact information for data requests

### CSS Components (`tailwind/base.css`)
Custom CSS classes for consent system:
- `.consent-btn-accept`: Green accept button matching site theme
- `.consent-btn-reject`: Gray reject button
- `.privacy-toggle-btn`: Toggle button in privacy modal

## Design System

### Color Scheme
- **Background**: Dark gray (`bg-gray-700`)
- **Text**: White on dark background
- **Accent**: Green (`green-500` / `green-400`) for links and primary actions
- **Secondary**: Gray tones for secondary actions

### Typography
- Base font styling in `tailwind/base.css`
- Responsive design with Tailwind utilities
- Consistent heading hierarchy

## Important Implementation Notes

### GDPR Compliance Requirements
1. **Consent must be freely given**: No cookie walls, site works without consent
2. **Withdrawal must be as easy as giving consent**: Privacy Settings always accessible in footer
3. **Consent expires**: 12-month automatic expiration with re-prompting
4. **Clear information**: Privacy policy explains what data is collected and why

### LocalStorage Usage
The consent system relies on browser localStorage:
- **Persistent**: Data survives browser restarts
- **Domain-specific**: Isolated per domain
- **User-controlled**: Users can clear via browser settings
- **No server dependency**: Purely client-side consent management

### PostHog Integration Best Practices
- **EU Instance**: Uses `eu.i.posthog.com` for EU data residency
- **Privacy Settings**: Configured for minimal data collection
- **Conditional Loading**: Only loads with explicit user consent
- **Public API Key**: Safe to expose in frontend code

### Development Workflow
1. Make changes to `tailwind/base.css` for styling updates
2. Run `npm run build-css-prod` to compile CSS
3. Test consent flow in incognito mode to simulate first-time visitors
4. Verify PostHog only loads after consent acceptance
5. Test privacy settings modal for consent withdrawal

This implementation provides a solid foundation for privacy-compliant analytics while maintaining a good user experience.

## AI Demo Architecture

The site is being extended with interactive AI demonstrations following a standardized architecture pattern.

### Demo Project Structure
Each AI demo is implemented as a separate repository with the following structure:
```
demo-project-name/
├── Cargo.toml                 # Workspace root
├── training/                  # Native training binary
├── interactive/               # Bevy WASM demo binary
├── shared/                    # Common model definitions
└── models/                    # Generated .safetensors files
```

### Technology Stack for Demos
- **Framework**: Bevy compiled to WASM for all interactive components
- **AI Inference**: Candle framework for consistent model operations
- **Model Format**: .safetensors for security and consistency
- **Build**: wasm-pack for WASM compilation

### Integration with Main Site
- WASM files served from `static/wasm/project-name/`
- Model files served from `static/models/project-name/`
- Demos accessible at `/demos/project-name` URLs
- Iframe embedding for flexible blog integration
- Consistent loading patterns and error handling

### Development Workflow for Demos
1. Train models with `cargo run --bin training` in demo repository
2. Build WASM with `wasm-pack build interactive --target web`
3. Copy generated assets to main site's static directories
4. Deploy through existing Docker pipeline

This standardized approach ensures consistent development patterns, code reuse opportunities, and simplified maintenance across multiple AI demonstration projects.