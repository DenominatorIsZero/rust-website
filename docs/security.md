# Security Documentation

Security configuration and decisions for the website.

## Content Security Policy

```
Content-Security-Policy: default-src 'self'; script-src 'self' https://eu.i.posthog.com https://eu-assets.i.posthog.com 'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data:; connect-src 'self' https://eu.i.posthog.com https://eu-assets.i.posthog.com; font-src 'self'; object-src 'none'; base-uri 'self'; form-action 'self'; frame-src 'self'; upgrade-insecure-requests
```

### CSP Directives

**`default-src 'self'`**
- Default fallback for all resource types
- Only allows resources from same origin

**`script-src 'self' https://eu.i.posthog.com https://eu-assets.i.posthog.com 'wasm-unsafe-eval'`**
- `'self'`: Scripts from `/static/` directory
- PostHog domains: Analytics functionality
- `'wasm-unsafe-eval'`: Enables WebAssembly instantiation

**`style-src 'self'`**
- Only allows CSS from same origin
- No inline styles permitted

**`img-src 'self' data:`**
- `'self'`: Images from static directory
- `data:`: Allows data URIs for small embedded images

**`connect-src 'self' https://eu.i.posthog.com https://eu-assets.i.posthog.com`**
- Controls XHR, WebSocket, fetch() destinations
- PostHog domains: Analytics data transmission

**`font-src 'self'`**
- Only fonts from same origin
- No external font services

**`object-src 'none'`**
- Disables plugins (Flash, etc.)
- Eliminates plugin vulnerabilities

**`base-uri 'self'`**
- Prevents malicious base tag injection
- Ensures relative URLs resolve correctly

**`form-action 'self'`**
- Forms only submit to same origin
- Prevents CSRF via form redirection

**`frame-src 'self'`**
- Allows same-origin iframes (required for AI demos)
- Blocks external iframe sources

**`upgrade-insecure-requests`**
- Converts HTTP requests to HTTPS
- Works with HSTS

### Known Issue: JavaScript eval()

**Problem**: WASM demos generate CSP violations:

```
Content-Security-Policy: The page's settings blocked a JavaScript eval (script-src)
```

**Location**: `static/wasm/example-project/interactive.js:935`

```javascript
imports.wbg.__wbg_eval_e10dc02e9547f640 = function () {
  return handleError(function (arg0, arg1) {
    const ret = eval(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
  }, arguments);
};
```

**Current Status**:

- Demos work fine (eval is blocked but has fallbacks)
- Console shows CSP violation
- No functional impact

**Future Options**:

1. **Rebuild WASM without eval** (preferred)
2. **Add `'unsafe-eval'`** (security risk)
3. **Route-specific CSP** (complex)

## HTTP Security Headers

### X-Frame-Options: SAMEORIGIN

- Prevents external clickjacking
- Allows same-origin iframes for AI demos

### X-Content-Type-Options: nosniff

- Prevents MIME type confusion attacks
- Forces strict Content-Type respect

### Referrer-Policy: strict-origin-when-cross-origin

- Same-origin: full referrer
- Cross-origin HTTPS: origin only
- HTTPS→HTTP: no referrer

### Strict-Transport-Security

- `max-age=31536000; includeSubDomains`
- Enforces HTTPS for 1 year
- Includes all subdomains

## Analytics

### PostHog Configuration

```javascript
posthog.init('phc_oPXS4I08n2TakqavgbpWkH16iME2iE2j94eTzpaj4tp', {
  api_host: 'https://eu.i.posthog.com',
  person_profiles: 'identified_only',
  disable_session_recording: true,
  respect_dnt: true,
  capture_pageview: true,
  capture_pageleave: false,
});
```

### GDPR Compliance

- EU data residency (`eu.i.posthog.com`)
- Explicit consent required
- 12-month consent expiration
- localStorage: `analytics-consent`, `analytics-consent-date`

## Docker Security

### Base Images

- Builder: `rust:1.89-slim`
- Runtime: `gcr.io/distroless/cc-debian12:nonroot`

### Security Features

- Non-root execution (`nonroot` user, UID 65532)
- File ownership: `--chown=nonroot:nonroot`
- Minimal attack surface (distroless)

### Template Security

```rust
tera.autoescape_on(vec![".html", ".sql"]);
```

## Adding New Demos

1. Build: `wasm-pack build --target web`
2. Place in `static/wasm/demo-name/`
3. Test for eval() CSP violations
4. If demo breaks without eval, consider rebuilding WASM or adding `'unsafe-eval'`

## Implementation

**CSP location**: `src/lib.rs`
**JavaScript**: External modules, no inline code
**Templates**: Auto-escaping enabled
