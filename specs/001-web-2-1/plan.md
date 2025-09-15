# Implementation Plan: Personal Memory Memo Application

**Branch**: `001-web-2-1` | **Date**: 2025-09-07 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-web-2-1/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → LOADED: Personal Memory Memo Application spec
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Project Type: web (frontend+backend)
   → Set Structure Decision: Option 2 (Web application)
3. Evaluate Constitution Check section below
   → Initial Constitution Check: PASS (2 projects, simple architecture)
   → Update Progress Tracking: Initial Constitution Check
4. Execute Phase 0 → research.md
   → Research completed for Rust web stack, authentication, database choices
5. Execute Phase 1 → contracts, data-model.md, quickstart.md, CLAUDE.md
   → All artifacts generated based on functional requirements
6. Re-evaluate Constitution Check section
   → Post-Design Constitution Check: PASS (maintains simplicity)
   → Update Progress Tracking: Post-Design Constitution Check
7. Plan Phase 2 → Task generation approach described
8. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
Web-based memo application with two types of notes (forever_memo and flush_memo) and user authentication. Rust backend with web frontend, targeting <200ms response times and deployment to https://rikumiura.com/memory_memo.

## Technical Context
**Language/Version**: Rust 1.75+ (backend), HTML/CSS/JavaScript (frontend)
**Primary Dependencies**: Axum (web framework), SQLite/PostgreSQL (database), Tokio (async runtime), bcrypt (password hashing)
**Storage**: PostgreSQL for production, SQLite for development
**Testing**: cargo test (Rust), basic JS testing for frontend
**Target Platform**: Linux server deployment
**Project Type**: web (frontend + backend)
**Performance Goals**: <200ms page loads, instant memo operations, automatic flush memo cleanup
**Constraints**: <200ms p95 response time, mobile-first responsive design, WCAG 2.1 AA compliance
**Scale/Scope**: Small personal app, <10k users expected, unlimited memos per user

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Simplicity**:
- Projects: 2 (backend API, frontend web)
- Using framework directly? YES (Axum directly, no wrapper classes)
- Single data model? YES (User, ForeverMemo, FlushMemo - no DTOs)
- Avoiding patterns? YES (direct DB access, no Repository/UoW pattern)

**Architecture**:
- EVERY feature as library? YES (auth-lib, memo-lib, cleanup-lib)
- Libraries listed: [auth-lib: user registration/login, memo-lib: CRUD operations, cleanup-lib: flush memo expiry]
- CLI per library: [auth-cli --register/--verify, memo-cli --create/--list, cleanup-cli --run]
- Library docs: llms.txt format planned? YES

**Testing (NON-NEGOTIABLE)**:
- RED-GREEN-Refactor cycle enforced? YES (tests written first)
- Git commits show tests before implementation? YES
- Order: Contract→Integration→E2E→Unit strictly followed? YES
- Real dependencies used? YES (actual PostgreSQL/SQLite)
- Integration tests for: new libraries, contract changes, shared schemas? YES
- FORBIDDEN: Implementation before test, skipping RED phase

**Observability**:
- Structured logging included? YES (tracing crate)
- Frontend logs → backend? YES (unified error reporting)
- Error context sufficient? YES (detailed error types)

**Versioning**:
- Version number assigned? YES (1.0.0 - MAJOR.MINOR.BUILD)
- BUILD increments on every change? YES
- Breaking changes handled? YES (API versioning planned)

## Project Structure

### Documentation (this feature)
```
specs/001-web-2-1/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/
```

**Structure Decision**: Option 2 (Web application) - frontend and backend separation required

## Phase 0: Outline & Research

1. **Extract unknowns from Technical Context** above:
   - Rust web framework choice (Axum vs actix-web vs warp)
   - Database choice for production (PostgreSQL vs SQLite)
   - Authentication strategy (session vs JWT)
   - Frontend framework decision (vanilla JS vs lightweight framework)
   - Deployment strategy for https://rikumiura.com/memory_memo

2. **Generate and dispatch research agents**:
   ```
   Task: "Research Rust web frameworks for simple memo app with <200ms response requirements"
   Task: "Find best practices for user authentication in Rust web applications"
   Task: "Research database choices for personal memo applications with automatic cleanup"
   Task: "Find minimal frontend approaches for Rust web backends"
   Task: "Research deployment patterns for Rust web apps on personal domains"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with all technology choices resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - User: id, username, password_hash, created_at
   - ForeverMemo: id, user_id, content, created_at
   - FlushMemo: id, user_id, content, created_at, expires_at

2. **Generate API contracts** from functional requirements:
   - POST /auth/register (user creation)
   - POST /auth/login (authentication)
   - POST /auth/logout (session cleanup)
   - GET /memos (list user memos)
   - POST /memos/forever (create forever memo)
   - POST /memos/flush (create flush memo)
   - DELETE /memos/{id} (delete memo)
   - Output OpenAPI schema to `/contracts/`

3. **Generate contract tests** from contracts:
   - One test file per endpoint
   - Assert request/response schemas
   - Tests must fail (no implementation yet)

4. **Extract test scenarios** from user stories:
   - User registration and login flow
   - Creating and viewing memos
   - Flush memo expiration behavior
   - UI responsive design validation

5. **Update agent file incrementally** (O(1) operation):
   - Run `/scripts/update-agent-context.sh claude` for Claude Code
   - Add Rust, Axum, PostgreSQL context
   - Include memo app domain knowledge
   - Update recent architectural decisions

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, CLAUDE.md

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `/templates/tasks-template.md` as base
- Generate tasks from Phase 1 design docs (contracts, data model, quickstart)
- Each contract → contract test task [P]
- Each entity → model creation task [P] 
- Each user story → integration test task
- Implementation tasks to make tests pass

**Ordering Strategy**:
- TDD order: Tests before implementation 
- Dependency order: Models before services before API before UI
- Mark [P] for parallel execution (independent files)

**Estimated Output**: 25-30 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)  
**Phase 4**: Implementation (execute tasks.md following constitutional principles)  
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*No constitutional violations identified - simple 2-project web application*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | - | - |

## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command) ✅
- [x] Phase 1: Design complete (/plan command) ✅  
- [x] Phase 2: Task planning complete (/plan command - describe approach only) ✅
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS ✅
- [x] Post-Design Constitution Check: PASS ✅
- [x] All NEEDS CLARIFICATION resolved ✅
- [x] Complexity deviations documented ✅

**Artifacts Generated**:
- [x] research.md: Technology stack decisions documented
- [x] data-model.md: User, ForeverMemo, FlushMemo entities defined
- [x] contracts/api-spec.yaml: Complete OpenAPI specification
- [x] quickstart.md: User journey validation steps
- [x] CLAUDE.md: Agent context updated with project details

---
*Based on Memory Memo Constitution v1.0.0 - See `/memory/constitution.md`*