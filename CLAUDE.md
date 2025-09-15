# Memory Memo Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-09-07

## Active Technologies
- Rust 1.75+ (backend framework: Axum)
- SQLite (database with SQLx)
- HTML/CSS/JavaScript (frontend, vanilla JS + HTMX)
- Session-based authentication
- Tokio async runtime
- bcrypt password hashing
- tracing logging

## Project Structure
```
backend/
├── src/
│   ├── models/      # User, ForeverMemo, FlushMemo
│   ├── services/    # Auth, memo CRUD, cleanup
│   └── api/         # Axum route handlers
└── tests/
    ├── contract/    # API contract tests
    ├── integration/ # End-to-end user flows  
    └── unit/        # Individual function tests

frontend/
├── src/
│   ├── components/  # UI components
│   ├── pages/       # Login, dashboard pages
│   └── services/    # API communication
└── tests/
```

## Commands
```bash
# Backend development
cd backend && cargo run
cargo test

# Database setup
sqlx migrate run

# Frontend development  
cd frontend && python -m http.server 8080
```

## Code Style
- Rust: Follow standard conventions with rustfmt
- Frontend: Vanilla JavaScript, minimal dependencies
- Design: Inter (English) + Noto Sans JP (Japanese) fonts
- Minimal flat design, large whitespace, few colors

## Constitutional Requirements
- <200ms response times (performance critical)
- TDD: Tests before implementation (RED-GREEN-Refactor)
- Library-first: auth-lib, memo-lib, cleanup-lib
- Simplicity: Direct framework usage, no wrapper patterns
- Privacy: Session auth, user isolation, no tracking

## Domain Context
Personal memo app with two types:
- ForeverMemo: Persistent until manually deleted
- FlushMemo: Auto-expires after 24 hours, displayed above forever memos
- User authentication with unique usernames
- Mobile-first responsive design

## Recent Changes
- 001-web-2-1: Initial implementation plan with Axum + SQLite + vanilla JS stack

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->