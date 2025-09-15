# Quickstart: Personal Memory Memo Application

**Purpose**: Validate the complete user journey from account creation to memo management  
**Prerequisites**: Backend and frontend implementations complete  
**Estimated time**: 5 minutes

## User Story Validation

### 1. Account Creation & Login Flow
```bash
# Test user registration
curl -X POST https://rikumiura.com/memory_memo/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username": "testuser", "password": "testpassword123"}'

# Expected: 201 Created with user details
# Expected response: {"id": "uuid", "username": "testuser", "created_at": "timestamp"}
```

```bash
# Test user login
curl -X POST https://rikumiura.com/memory_memo/api/auth/login \
  -H "Content-Type: application/json" \
  -c cookies.txt \
  -d '{"username": "testuser", "password": "testpassword123"}'

# Expected: 200 OK with session cookie set
# Expected response: {"user_id": "uuid", "username": "testuser"}
```

### 2. Memo Creation & Management
```bash
# Create a forever memo
curl -X POST https://rikumiura.com/memory_memo/api/memos/forever \
  -H "Content-Type: application/json" \
  -b cookies.txt \
  -d '{"content": "This is my first forever memo"}'

# Expected: 201 Created
# Expected response: {"id": "uuid", "content": "This is my first forever memo", "created_at": "timestamp"}
```

```bash
# Create a flush memo
curl -X POST https://rikumiura.com/memory_memo/api/memos/flush \
  -H "Content-Type: application/json" \
  -b cookies.txt \
  -d '{"content": "This flush memo expires in 24 hours"}'

# Expected: 201 Created  
# Expected response: {"id": "uuid", "content": "This flush memo expires in 24 hours", "created_at": "timestamp", "expires_at": "timestamp+24h"}
```

```bash
# List all memos (should show flush memos first, then forever memos)
curl -X GET https://rikumiura.com/memory_memo/api/memos \
  -b cookies.txt

# Expected: 200 OK with flush memos array first, forever memos array second
# Expected: Newest items first within each category
```

### 3. Web Interface Validation

Navigate to `https://rikumiura.com/memory_memo` and verify:

#### Registration/Login UI
- [ ] Registration form accepts username (3-50 chars) and password (8+ chars)
- [ ] Error handling for duplicate username
- [ ] Successful login redirects to memo dashboard
- [ ] Login form shows appropriate error messages

#### Memo Dashboard UI
- [ ] Flush memo section appears above forever memo section
- [ ] New memo creation forms for both types
- [ ] Memos display newest first within each section
- [ ] Clean, minimal design with Inter/Noto Sans JP fonts
- [ ] Large whitespace and few colors (constitutional requirement)

#### Memo Management
- [ ] Create forever memo: content appears immediately at top of forever section
- [ ] Create flush memo: content appears immediately at top of flush section
- [ ] Delete functionality works for both memo types
- [ ] Empty state displays appropriately when no memos exist

#### Responsive Design
- [ ] Mobile-first design works on small screens
- [ ] Text remains readable on all screen sizes
- [ ] Touch targets are appropriately sized
- [ ] Accessibility: WCAG 2.1 AA compliance

### 4. Performance Validation

Verify constitutional performance requirements:

```bash
# Test page load time (should be <200ms)
curl -o /dev/null -s -w "Total time: %{time_total}s\n" https://rikumiura.com/memory_memo

# Expected: <0.2 seconds
```

```bash
# Test memo creation response time
time curl -X POST https://rikumiura.com/memory_memo/api/memos/forever \
  -H "Content-Type: application/json" \
  -b cookies.txt \
  -d '{"content": "Performance test memo"}'

# Expected: <200ms total time
```

### 5. Automatic Cleanup Validation

**24-hour flush memo expiration test**:
1. Create a flush memo
2. Note the `expires_at` timestamp  
3. Wait for system cleanup process (or trigger manually)
4. Verify memo no longer appears in `/memos` endpoint after expiration

### 6. Security Validation

```bash
# Test unauthenticated access (should fail)
curl -X GET https://rikumiura.com/memory_memo/api/memos

# Expected: 401 Unauthorized
```

```bash
# Test access to another user's memos (should fail)
# (Requires creating second user account first)
curl -X GET https://rikumiura.com/memory_memo/api/memos \
  -b other_user_cookies.txt

# Expected: Only returns memos for the authenticated user
```

## Success Criteria

This quickstart passes if:
- [ ] All API endpoints return expected status codes and response formats
- [ ] Web interface displays memos in correct order (flush first, newest first within type)
- [ ] Performance meets <200ms constitutional requirement
- [ ] Users can only access their own memos
- [ ] Flush memos expire after 24 hours
- [ ] UI follows design language (Inter/Noto Sans JP, minimal flat design)
- [ ] Mobile responsiveness and accessibility requirements met

## Troubleshooting

**Common issues**:
- **401 errors**: Check session cookie is being sent with requests
- **Slow responses**: Verify database indexes are created correctly
- **UI layout issues**: Confirm CSS is loading and fonts are available
- **Flush memo cleanup**: Verify background cleanup process is running

This quickstart validates all functional requirements from the specification and ensures constitutional compliance.