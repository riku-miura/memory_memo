#!/bin/bash

# Memory Memo Database Restore Script
set -e

echo "🔄 Memory Memo Database Restore"
echo "=============================="

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if backup file is provided
if [ -z "$1" ]; then
    print_error "Please provide backup file path"
    echo "Usage: $0 <backup_file>"
    echo ""
    print_info "Available backups:"
    ls -la ./backups/memory_memo_backup_*.db 2>/dev/null || echo "No backups found"
    exit 1
fi

BACKUP_FILE=$1
CONTAINER_NAME="memory_memo_memory-memo_1"

# Check if backup file exists
if [ ! -f "$BACKUP_FILE" ]; then
    print_error "Backup file not found: $BACKUP_FILE"
    exit 1
fi

print_info "Restoring from: $BACKUP_FILE"

# Confirmation
print_warning "This will overwrite the current database!"
print_warning "Make sure to backup current data if needed."
echo -n "Continue? (y/N): "
read -r CONFIRM

if [ "$CONFIRM" != "y" ] && [ "$CONFIRM" != "Y" ]; then
    print_info "Restore cancelled"
    exit 0
fi

# Check if container is running
if docker ps | grep -q $CONTAINER_NAME; then
    print_info "Stopping application container..."
    docker-compose down
    
    # Wait a moment
    sleep 2
    
    print_info "Restoring database in container..."
    
    # Start container with temporary override
    docker-compose up -d
    
    # Wait for container to be ready
    sleep 5
    
    # Copy backup to container and restore
    docker cp "$BACKUP_FILE" $CONTAINER_NAME:/tmp/restore.db
    docker exec $CONTAINER_NAME sh -c "rm -f /app/data/memory_memo.db && cp /tmp/restore.db /app/data/memory_memo.db"
    docker exec $CONTAINER_NAME rm /tmp/restore.db
    
    # Restart container to reload database
    docker-compose restart
    
    print_status "Database restored in container"
    
else
    # Restore to local file
    if [ -f "backend/memory_memo.db" ]; then
        print_info "Backing up current local database..."
        mv backend/memory_memo.db "backend/memory_memo.db.backup.$(date +%Y%m%d_%H%M%S)"
    fi
    
    cp "$BACKUP_FILE" backend/memory_memo.db
    print_status "Database restored locally"
fi

print_info "Verifying restore..."

# Wait for application to be ready
sleep 5

# Health check
if curl -f -s http://localhost:3000/health > /dev/null; then
    print_status "Application is running and healthy!"
else
    print_warning "Application may not be running. Please check manually."
fi

print_status "Database restore completed!"
print_info "Restored from: $BACKUP_FILE"