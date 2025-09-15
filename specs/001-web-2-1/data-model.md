# Data Model: Personal Memory Memo Application

**Date**: 2025-09-07  
**Based on**: Feature specification functional requirements

## Entity Definitions

### User
Represents a registered user account with unique credentials.

**Fields**:
- `id`: UUID (Primary Key) - Unique user identifier
- `username`: String (Unique, NOT NULL, max 50 chars) - User's login identifier
- `password_hash`: String (NOT NULL) - bcrypt hashed password
- `created_at`: DateTime (NOT NULL) - Account creation timestamp

**Validation Rules**:
- Username must be unique across all users (FR-002)
- Username length: 3-50 characters
- Password minimum 8 characters before hashing
- No email validation required (username/password only)

**Relationships**:
- One-to-many: User → ForeverMemo
- One-to-many: User → FlushMemo

### ForeverMemo
Persistent memo content that remains until manually deleted.

**Fields**:
- `id`: UUID (Primary Key) - Unique memo identifier  
- `user_id`: UUID (Foreign Key, NOT NULL) - References User.id
- `content`: Text (NOT NULL) - Memo content (no artificial length limit)
- `created_at`: DateTime (NOT NULL) - Creation timestamp for ordering

**Validation Rules**:
- Content cannot be empty string
- User can only access their own memos (FR-008)
- No title or explicit timestamps shown to user (FR-004)
- Newest items displayed first within category (FR-007)

**Relationships**:
- Many-to-one: ForeverMemo → User

### FlushMemo
Temporary memo content that expires after 24 hours.

**Fields**:
- `id`: UUID (Primary Key) - Unique memo identifier
- `user_id`: UUID (Foreign Key, NOT NULL) - References User.id  
- `content`: Text (NOT NULL) - Memo content (no artificial length limit)
- `created_at`: DateTime (NOT NULL) - Creation timestamp
- `expires_at`: DateTime (NOT NULL) - Expiration timestamp (created_at + 24 hours)

**Validation Rules**:
- Content cannot be empty string
- Expires exactly 24 hours after creation (FR-005, FR-009)
- User can only access their own memos (FR-008)
- Displayed above forever memos in UI (FR-006)
- Newest items displayed first within category (FR-007)
- Automatically removed by system cleanup process (FR-009)

**Relationships**:
- Many-to-one: FlushMemo → User

## Database Schema (SQLite)

```sql
-- Users table
CREATE TABLE users (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    username TEXT UNIQUE NOT NULL CHECK(length(username) >= 3 AND length(username) <= 50),
    password_hash TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Forever memos table  
CREATE TABLE forever_memos (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT NOT NULL,
    content TEXT NOT NULL CHECK(length(content) > 0),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

-- Flush memos table
CREATE TABLE flush_memos (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT NOT NULL,
    content TEXT NOT NULL CHECK(length(content) > 0),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    expires_at DATETIME NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX idx_forever_memos_user_created ON forever_memos(user_id, created_at DESC);
CREATE INDEX idx_flush_memos_user_created ON flush_memos(user_id, created_at DESC);
CREATE INDEX idx_flush_memos_expires ON flush_memos(expires_at);
```

## State Transitions

### FlushMemo Lifecycle
1. **Created**: expires_at = created_at + 24 hours
2. **Active**: visible to user, editable through UI
3. **Expired**: automatically removed by cleanup process
4. **Deleted**: removed from database (no recovery)

### User Session States
1. **Anonymous**: no access to memo functionality
2. **Authenticated**: full access to own memos only
3. **Logged Out**: session invalidated, return to anonymous

## Data Access Patterns

### Query Patterns
- **User memos list**: ORDER BY memo type (flush first), then created_at DESC within type
- **Cleanup process**: WHERE expires_at < CURRENT_TIMESTAMP for flush memos
- **User isolation**: WHERE user_id = ? for all memo operations

### Performance Considerations
- Indexes on user_id + created_at for efficient memo listing
- Index on expires_at for efficient cleanup operations
- UUID primary keys avoid sequence contention
- CASCADE DELETE ensures referential integrity

This data model satisfies all functional requirements while maintaining simplicity and supporting the constitutional performance requirements (<200ms response times).