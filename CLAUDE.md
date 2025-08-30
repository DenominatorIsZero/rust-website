# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with this website repository.

## Project Overview

This is a personal website/blog built with Rust using the Actix-web framework. The site serves academic publications and blog posts with content sourced from TOML frontmatter files and Markdown content. The site includes GDPR-compliant analytics tracking using PostHog with a comprehensive consent management system.

**Repository**: Personal website with AI demo integration capabilities

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

## Development Guidelines

### Code Organization

- **Handlers**: Organize by functionality in `src/handlers/` (home, blog, post, publications, privacy)
- **Templates**: HTML templates in `templates/` using Tera templating engine
- **Static Assets**: CSS in `tailwind/base.css`, compiled to `static/css/index.css`
- **Content Management**: Posts in `posts/`, publications in `publications/` with TOML frontmatter
- **Configuration**: Main application setup in `src/lib.rs`, server entry in `src/main.rs`

### Website Development Patterns

- **Content Discovery**: Use file-walking approach for posts and publications
- **Template Structure**: Follow existing base.html → specific template inheritance
- **CSS Development**: Work in `tailwind/base.css`, compile with `npm run build-css-prod`
- **Privacy Compliance**: Maintain GDPR compliance for any user-facing features
- **Analytics Integration**: Follow existing PostHog consent management patterns

### Quality Standards

- Follow existing code style and architectural patterns
- Ensure mobile responsiveness for all UI changes
- Test Docker builds regularly for deployment readiness
- Maintain privacy compliance and GDPR requirements
- Verify PostHog analytics integration works correctly

## AI-Assisted Development Workflow

This project follows a systematic AI-assisted development workflow designed for efficient collaboration with Claude Code. The workflow ensures well-documented, systematic feature development from initial concept to final implementation.

### Documentation Structure

The `docs/` folder supports the AI-assisted development process:

- **`docs/projects/`** - Project-based documentation organization
  - **`archive/`** - Completed projects with their specifications and implementation plans
  - **`template/`** - Reusable specification and plan templates for new website features
    - `spec-template.md` - Website feature specification template
    - `plan-template.md` - Phase-based implementation plan template
    - `README.md` - Template usage guidelines for website development

### Three-Phase Development Process

#### Phase 1: Specification
Work with Claude Code to develop comprehensive feature specifications:

1. **Problem Definition** - Clearly articulate what needs to be built and why
2. **Requirements Gathering** - Define functional and non-functional requirements
3. **Success Criteria** - Establish measurable outcomes and acceptance criteria
4. **Technical Constraints** - Document limitations, dependencies, and architectural considerations
5. **Website Integration** - Define integration points with existing website systems

The specification phase is complete when:
- All stakeholders would understand what's being built
- Technical approach is well-defined and fits existing architecture
- Edge cases and error scenarios are covered
- Success criteria are measurable and testable
- Privacy/GDPR implications are considered

#### Phase 2: Planning
Transform specifications into actionable implementation plans:

1. **Task Breakdown** - Decompose features into discrete, atomic todos
2. **Dependency Mapping** - Identify task dependencies and optimal sequencing
3. **Website Integration Strategy** - Plan integration with templates, handlers, and static assets
4. **Commit Strategy** - Plan natural checkpoints for incremental commits
5. **Quality Gates** - Define testing and validation at each checkpoint

Plans should contain:
- Numbered, sequential tasks that can be completed independently
- Clear definition of "done" for each task including website-specific validation
- Commit points that represent stable, testable states
- Docker build and deployment considerations
- Use `[ ]` checkboxes for trackable tasks, `[x]` for completed tasks

#### Phase 3: Execution
Implement plans step-by-step with Claude Code assistance:

1. **Task-by-Task Implementation** - Work through planned todos systematically
2. **Frequent Commits** - Commit at natural checkpoints for incremental progress
3. **Continuous Validation** - Test changes with `cargo run` and verify Docker builds
4. **Documentation Updates** - Keep documentation in sync with implementation
5. **Plan Adaptation** - Adjust plans based on discoveries during implementation

### AI Integration Guidelines

#### Effective Collaboration with Claude Code

- **Context Management** - Reference specifications and plans to maintain context across sessions
- **Incremental Development** - Work on one discrete task at a time
- **Code Review Partnership** - Use Claude Code to review implementations before committing
- **Problem-Solving** - Leverage Claude Code's analysis for debugging and optimization
- **Website Patterns** - Follow existing website architecture and coding conventions

#### Best Practices for AI-Assisted Website Development

- **Clear Communication** - Provide specific, actionable requests
- **Iterative Refinement** - Use multiple rounds of feedback to improve specifications and implementations
- **Documentation First** - Always document decisions and rationale for future reference
- **Validation Focus** - Ask Claude Code to help validate implementations against specifications
- **Privacy Awareness** - Ensure GDPR compliance considerations are addressed
- **Mobile Testing** - Verify responsive design for all UI changes

#### CLAUDE.md Maintenance

**IMPORTANT**: Keep this file current to improve future AI interactions.

**When to update CLAUDE.md:**
- After major architectural changes or refactoring
- When new development patterns or workflows are established
- After discovering common issues and their solutions
- When user has to correct AI assumptions or provide missing context
- After adding new tools, dependencies, or build processes
- After significant website feature additions or privacy compliance updates

**Proactive Update Protocol:**
- Claude Code should offer to update CLAUDE.md after significant changes
- Focus on generic guidance that benefits future interactions
- Document new patterns, workflows, or common corrections
- Update file paths, command references, or architectural descriptions
- Add lessons learned from debugging or problem-solving sessions

Example trigger situations:
- "You should use X instead of Y" → Add to development guidelines
- "The files are actually located in Z" → Update repository structure
- "This command doesn't work, use this instead" → Update development commands
- Major refactoring or new features → Update relevant sections

### Commit Strategy

#### Commit as Natural Checkpoints

Commits should represent meaningful progress points, not arbitrary code changes:

- **Feature Milestones** - Complete implementation of planned tasks
- **Stable States** - Code compiles, tests pass, and functionality works
- **Logical Units** - Related changes grouped together (e.g., handler + template + CSS)
- **Rollback Points** - States you could confidently return to if needed

#### Commit Review Requirement

**IMPORTANT**: Always ask the user to review changes before committing. Present a summary of what will be committed and wait for approval before executing any git commit commands.

#### Commit Message Conventions

Follow this format for AI workflow commits:

```
[PHASE] Brief description of change

- Specific changes made
- Reference to docs/projects/ files
- Any deviations from original plan

Closes: #issue-number (if applicable)
Refs: docs/projects/feature-name/specs/feature-name.md, docs/projects/feature-name/plan.md
```

Examples:
```
[SPEC] Define contact form requirements

- Added comprehensive contact form spec with validation
- Defined email integration and spam protection
- Identified integration points with existing privacy system

Refs: docs/projects/contact-form/specs/contact-form.md
```

```
[PLAN] Break down contact form implementation

- Created 8 discrete tasks for form implementation
- Identified dependencies on existing template system
- Planned email handler and validation logic

Refs: docs/projects/contact-form/plan.md
```

```
[IMPL] Add contact form handler and template

- Implemented /contact route with form processing
- Added contact form template with validation
- Integrated with existing privacy and analytics systems
- Added comprehensive error handling and user feedback

Refs: docs/projects/contact-form/specs/contact-form.md, docs/projects/contact-form/plan.md
```

```
[DOCS] Update architecture documentation for contact system

- Added contact form flow diagrams and validation details
- Updated handler documentation with new endpoints
- Documented email integration and spam protection
- Added troubleshooting section for common form issues

Refs: CLAUDE.md
```

```
[MAINT] Update dependencies and fix clippy warnings

- Updated Actix-web to latest version for security improvements
- Fixed deprecated function calls in handlers
- Enhanced CSS build process for better performance
- Updated Docker configuration for optimized builds

Refs: Cargo.toml, tailwind/package.json, Dockerfile
```

#### Branching Strategy

- **Feature Branches** - One branch per specification for organized development (e.g., `feature/contact-form`)
- **Direct Merges** - Merge to main after local validation and testing (solo project workflow)
- **Clean History** - Use meaningful commit messages with phase tags for clear development history
- **Rollback Safety** - Each commit represents a stable checkpoint you can confidently return to