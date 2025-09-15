# Memory Memo Constitution

## Core Principles

### I. Simplicity First
Every feature must serve a clear user need without unnecessary complexity. The application prioritizes ease of use over feature richness. New functionality is added only when it directly enhances the core memo experience without compromising the minimalist design philosophy.

### II. Data Privacy & Security
User data belongs to the user. Authentication is secure, memo content is protected, and users can only access their own data. No tracking, no analytics beyond essential system metrics, no data mining. Privacy by design, not by policy.

### III. Performance & Responsiveness
The application must feel instant. Page loads under 200ms, memo creation/editing without perceptible delay, flush memo cleanup without user impact. Performance regression is treated as a critical bug.

### IV. Clean Design Language
Minimal flat design with intentional typography using Inter (English) and Noto Sans JP (Japanese). Large whitespace, few colors, clear visual hierarchy. Design decisions prioritize readability and cognitive ease over visual complexity.

### V. Reliable Data Management
Forever memos persist reliably; flush memos expire predictably at 24 hours. No data loss, no unexpected behavior. Users trust the system to handle their thoughts consistently.

## Technical Standards

### Code Quality
- Rust safety principles: no unsafe blocks without explicit justification
- Clear error handling with meaningful user feedback
- Comprehensive testing for data persistence and user authentication
- Code reviews focus on security, performance, and maintability

### Deployment & Operations
- Production deployment to https://rikumiura.com/memory_memo
- Zero-downtime deployments
- Automated backup and recovery procedures
- Monitoring for performance and security incidents

## User Experience Standards

### Interface Design
- Mobile-first responsive design
- Keyboard shortcuts for power users
- Accessibility compliance (WCAG 2.1 AA)
- Consistent interaction patterns throughout the application

### Content Management
- Flush memos prominently displayed above forever memos
- Newest items first within each category
- No artificial limits on memo count or length (within reasonable system constraints)
- Clear visual distinction between memo types

## Governance

This constitution guides all development decisions. Feature requests, bug fixes, and technical improvements must align with these principles. When in doubt, choose the simpler solution that best serves the user's core need to capture and organize thoughts.

Amendments require:
1. Clear rationale for change
2. Impact assessment on existing users
3. Implementation plan that maintains backward compatibility

**Version**: 1.0.0 | **Ratified**: 2025-09-07 | **Last Amended**: 2025-09-07