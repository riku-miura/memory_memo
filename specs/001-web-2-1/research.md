# Research: Personal Memory Memo Application

**Date**: 2025-09-07  
**Purpose**: Technology stack research for Rust-based memo application

## Web Framework Selection

**Decision**: Axum  
**Rationale**: Optimal balance of performance, simplicity, and ecosystem integration. Provides <200ms response times with excellent developer experience and minimal memory footprint (12-20MB).  
**Alternatives considered**: 
- Actix-web: Highest raw performance but steeper learning curve
- Warp: Functional approach with good expressiveness but unique programming style

## Database Selection

**Decision**: SQLite  
**Rationale**: Perfect for personal memo applications - single file deployment, excellent performance for individual use, built-in automatic resource management with SQLx connection pooling. Meets <200ms requirements for personal scale.  
**Alternatives considered**: 
- PostgreSQL: Powerful for multi-user scenarios but overkill for personal memo app

## Authentication Strategy

**Decision**: Session-based authentication  
**Rationale**: Ideal for traditional web applications, immediate session control capability, straightforward CSRF protection implementation, excellent Rust library support (actix-session).  
**Alternatives considered**: 
- JWT: Stateless and scalable but unnecessarily complex for personal applications, better suited for microservices/native app APIs

## Frontend Approach

**Decision**: Vanilla JavaScript + HTMX  
**Rationale**: Maximum simplicity with no framework abstraction overhead, direct DOM manipulation for fast response, minimal dependencies for long-term maintenance ease, direct API endpoint communication.  
**Alternatives considered**: 
- Svelte: Compile-time optimization with lightweight output but excessive for memo app
- Preact: Lightweight React alternative but adds learning overhead

## Deployment Pattern

**Decision**: Docker + VPS + Caddy  
**Rationale**: Simple HTTPS auto-configuration with Caddy, cost-effective VPS for personal domains (Hetzner/DigitalOcean), Docker ensures environment consistency, GitHub Actions enables continuous deployment.  
**Alternatives considered**: 
- Direct VPS deployment: Lacks deployment consistency without containerization
- Cloud platforms: Cost-prohibitive for personal projects

## Technology Stack Summary

### Backend
- Framework: Axum
- Database: SQLite + SQLx
- Authentication: Session-based (actix-session)
- Runtime: Tokio
- Password hashing: bcrypt
- Logging: tracing

### Frontend
- JavaScript: Vanilla JS + HTMX
- Styling: CSS3 with Inter/Noto Sans JP fonts
- Design: Minimal flat design, large whitespace

### Infrastructure
- Containerization: Docker
- Reverse proxy: Caddy (automatic HTTPS)
- Hosting: VPS (Hetzner/DigitalOcean recommended)
- CI/CD: GitHub Actions
- Domain: https://rikumiura.com/memory_memo

## Performance Validation

This stack configuration supports:
- <200ms response times (constitutional requirement)
- Instant memo operations
- Automatic flush memo cleanup without user impact
- Mobile-first responsive design
- WCAG 2.1 AA accessibility compliance

All technical unknowns from the specification have been resolved with this research.