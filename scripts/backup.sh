#!/bin/bash

# Memory Memo Database Backup Script
set -e

echo "💾 Memory Memo Database Backup"
echo "============================="

# Configuration
BACKUP_DIR="./backups"
DATE=$(date +%Y%m%d_%H%M%S)
CONTAINER_NAME="memory_memo_memory-memo_1"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Create backup directory
mkdir -p $BACKUP_DIR

print_info "Creating database backup..."

# Check if container is running
if ! docker ps | grep -q $CONTAINER_NAME; then
    print_info "Container not running, looking for database file..."
    
    # Try to backup local database
    if [ -f "backend/memory_memo.db" ]; then
        cp backend/memory_memo.db "$BACKUP_DIR/memory_memo_backup_$DATE.db"
        print_status "Local database backed up to $BACKUP_DIR/memory_memo_backup_$DATE.db"
    else
        echo "No database file found to backup"
        exit 1
    fi
else
    # Backup from Docker container
    docker exec $CONTAINER_NAME sqlite3 /app/data/memory_memo.db ".backup /tmp/backup.db"
    docker cp $CONTAINER_NAME:/tmp/backup.db "$BACKUP_DIR/memory_memo_backup_$DATE.db"
    docker exec $CONTAINER_NAME rm /tmp/backup.db
    
    print_status "Database backed up from container to $BACKUP_DIR/memory_memo_backup_$DATE.db"
fi

# Show backup info
BACKUP_SIZE=$(du -h "$BACKUP_DIR/memory_memo_backup_$DATE.db" | cut -f1)
print_info "Backup size: $BACKUP_SIZE"

# Clean old backups (keep last 7 days)
find $BACKUP_DIR -name "memory_memo_backup_*.db" -type f -mtime +7 -delete 2>/dev/null || true

print_status "Backup completed successfully!"

# List recent backups
echo ""
print_info "Recent backups:"
ls -la $BACKUP_DIR/memory_memo_backup_*.db 2>/dev/null | tail -5 || echo "No previous backups found"