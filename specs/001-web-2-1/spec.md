# Feature Specification: Personal Memory Memo Application

**Feature Branch**: `001-web-2-1`  
**Created**: 2025-09-07  
**Status**: Draft  
**Input**: User description: "メモアプリです。Webアプリケーションとして動きます。メモは2種類あります。1つ目は、普通のメモでカードに記載できます。名前を「forever_memo」です。タイトルなどはつけられず、作成日時もなく、カードにメモが残っているだけのものを無数に作れるイメージです。新しいカードが常に上に来ます。2つ目は、1日で消えるメモです。名前は「flush_memo」です。「forever_memo」より上部に常に表示されます。こちらも複数作成できます。ユーザーごとにメモを作ることができるので、最初にアカウント作成して、ログインをできるようにしてください。IDやPASSに制約はないですが、ユーザーごとにユニークになるようにしてください。最終的にはhttps://rikumiura.com/memory_memoにデプロイする想定でお願いします。Rustで実装してほしい。ほかは技術的な制約はないです。Use \"Inter\" for English text and \"Noto Sans JP\" for Japanese text, minimal flat design, few colors, clean typography, large whitespace."

## Execution Flow (main)
```
1. Parse user description from Input
   → Parsed: Personal memo application with two types of notes and user accounts
2. Extract key concepts from description
   → Actors: Users, System
   → Actions: Create account, login, create/view/manage memos
   → Data: User accounts, forever_memo, flush_memo
   → Constraints: Unique usernames, 24-hour expiry for flush_memo, visual hierarchy
3. For each unclear aspect:
   → Marked authentication method, password requirements, data limits
4. Fill User Scenarios & Testing section
   → Defined primary user flows for account creation, memo management
5. Generate Functional Requirements
   → Created testable requirements for all core functionality
6. Identify Key Entities
   → User, Forever Memo, Flush Memo entities defined
7. Run Review Checklist
   → Contains clarification markers for authentication details
8. Return: SUCCESS (spec ready for planning, with clarifications needed)
```

---

## ⚡ Quick Guidelines
- ✅ Focus on WHAT users need and WHY
- ❌ Avoid HOW to implement (no tech stack, APIs, code structure)
- 👥 Written for business stakeholders, not developers

---

## User Scenarios & Testing *(mandatory)*

### Primary User Story
A user wants a simple, personal memo system where they can:
1. Create an account and securely access their personal space
2. Write quick, persistent notes (forever memos) that accumulate over time
3. Create temporary reminders (flush memos) that automatically disappear after one day
4. View all their memos in a clean, organized interface with temporary notes prominently displayed

### Acceptance Scenarios
1. **Given** no existing account, **When** user provides unique credentials, **Then** account is created and user can log in
2. **Given** logged-in user, **When** user creates a forever memo, **Then** memo appears at top of the forever memo section
3. **Given** logged-in user, **When** user creates a flush memo, **Then** memo appears in the flush memo section above all forever memos
4. **Given** flush memo older than 24 hours, **When** system runs cleanup, **Then** expired flush memos are automatically removed
5. **Given** multiple memos exist, **When** user views their memo dashboard, **Then** flush memos appear above forever memos, with newest items first in each section

### Edge Cases
- What happens when a user tries to register with an existing username?
- How does the system handle empty memo content?
- What occurs when a user has no active memos to display?
- How does the system behave if a user attempts to access another user's memos?

## Requirements *(mandatory)*

### Functional Requirements
- **FR-001**: System MUST allow users to create unique accounts with username and password
- **FR-002**: System MUST prevent duplicate usernames across all user accounts
- **FR-003**: System MUST authenticate users before allowing access to memo functionality
- **FR-004**: Users MUST be able to create unlimited forever memos without titles or timestamps
- **FR-005**: Users MUST be able to create multiple flush memos that automatically expire after 24 hours
- **FR-006**: System MUST display flush memos above forever memos in the user interface
- **FR-007**: System MUST order memos with newest items appearing first within each category
- **FR-008**: System MUST ensure users can only access their own memos
- **FR-009**: System MUST automatically remove flush memos after 24 hours from creation
- **FR-010**: System MUST persist forever memos indefinitely until user manually deletes them
- **FR-011**: System MUST provide authentication method [NEEDS CLARIFICATION: auth method not specified - email/password, username/password only?]
- **FR-012**: System MUST enforce password requirements [NEEDS CLARIFICATION: minimum length, complexity requirements?]
- **FR-013**: System MUST handle memo content limits [NEEDS CLARIFICATION: maximum memo length or character count?]

### Key Entities *(include if feature involves data)*
- **User Account**: Represents a registered user with unique identifier, password, and associated memos
- **Forever Memo**: Persistent note content belonging to a specific user, displayed in chronological order (newest first)
- **Flush Memo**: Temporary note content belonging to a specific user, automatically expires after 24 hours, displayed above forever memos

---

## Review & Acceptance Checklist
*GATE: Automated checks run during main() execution*

### Content Quality
- [x] No implementation details (languages, frameworks, APIs)
- [x] Focused on user value and business needs
- [x] Written for non-technical stakeholders
- [x] All mandatory sections completed

### Requirement Completeness
- [ ] No [NEEDS CLARIFICATION] markers remain
- [x] Requirements are testable and unambiguous  
- [x] Success criteria are measurable
- [x] Scope is clearly bounded
- [x] Dependencies and assumptions identified

---

## Execution Status
*Updated by main() during processing*

- [x] User description parsed
- [x] Key concepts extracted
- [x] Ambiguities marked
- [x] User scenarios defined
- [x] Requirements generated
- [x] Entities identified
- [ ] Review checklist passed

---