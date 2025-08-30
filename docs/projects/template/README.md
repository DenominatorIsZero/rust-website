# Project Templates

Generic templates for creating new website features, enhancements, and improvements using the established documentation and planning structure.

## Using the Templates

### For New Website Features

1. **Copy template files** to your new project directory:
   ```
   docs/projects/your-feature-name/
   ├── spec-template.md → your-feature-spec.md
   └── plan-template.md → plan.md
   ```

2. **Replace placeholders** in both files:
   - `[FEATURE_NAME]` → Your feature name (e.g., "Contact Form", "Search Functionality")
   - `[Brief description...]` → Your specific descriptions
   - `[X-Y] hours` → Your time estimates

3. **Customize sections** based on your feature type:
   - Select relevant website areas affected (Frontend, Backend, etc.)
   - Adjust implementation phases for your specific requirements
   - Update technical strategy for website context

### Template Structure

**`spec-template.md`** provides the website feature specification structure:
- Project overview and purpose for website context
- Website-specific implementation scope (templates, handlers, content)
- Success criteria including technical and UX requirements
- Integration considerations for existing website systems

**`plan-template.md`** provides the website implementation planning structure:
- Phase-based development approach adapted for web development
- Task breakdown with website-specific dependencies
- Quality gates including Docker builds and privacy compliance
- Deployment checklist for production readiness

### Website-Specific Adaptation Guidelines

#### Common Feature Types
- **Content Features**: New pages, blog enhancements, publication displays
- **Interactive Features**: Contact forms, search, user interactions
- **AI Demo Integration**: New demo pages, model serving, WASM integration
- **Privacy/Analytics**: GDPR compliance features, tracking enhancements
- **Infrastructure**: Build system, deployment, performance improvements

#### Website Integration Points
- **Templates**: Follow Tera templating patterns in `templates/`
- **Handlers**: Organize by functionality in `src/handlers/`
- **Static Assets**: CSS in `tailwind/`, other assets in `static/`
- **Content Management**: Posts in `posts/`, publications in `publications/`
- **Routing**: Central route configuration in `src/lib.rs`

#### Development Workflow Considerations
- Use `cargo watch -x run` for development iteration
- CSS changes require `npm run build-css-prod`
- Regular Docker build testing for deployment readiness
- Privacy compliance testing in incognito mode
- Mobile responsiveness verification

### Quality Standards

- Maintain existing code patterns and architecture
- Ensure GDPR compliance for any user-facing features
- Verify Docker build success for deployment
- Test mobile responsiveness for UI changes
- Follow existing CSS/design system patterns

These templates provide systematic structure for website feature development while ensuring consistency with existing architecture and quality standards.