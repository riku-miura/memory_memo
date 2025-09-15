#!/bin/bash

# Memory Memo Deployment Script
set -e

echo "🚀 Memory Memo Deployment Script"
echo "================================"

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    print_error "Docker is not installed. Please install Docker first."
    exit 1
fi

# Check for docker compose (new format) or $COMPOSE_CMD (legacy)
if ! (command -v $COMPOSE_CMD &> /dev/null || docker compose version &> /dev/null); then
    print_error "Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi

# Set compose command based on what's available
if command -v $COMPOSE_CMD &> /dev/null; then
    COMPOSE_CMD="$COMPOSE_CMD"
else
    COMPOSE_CMD="docker compose"
fi

print_status "Docker and Docker Compose are available"

# Check deployment mode
DEPLOYMENT_MODE=${1:-"development"}

if [ "$DEPLOYMENT_MODE" = "production" ]; then
    print_info "Deploying in PRODUCTION mode"
    
    # Check if .env file exists for production
    if [ ! -f ".env" ]; then
        print_warning "No .env file found. Creating from example..."
        cp .env.example .env
        print_warning "Please edit .env file with your production settings"
        print_warning "Press Enter to continue after editing .env file..."
        read
    fi
    
    # Use production $COMPOSE_CMD override
    COMPOSE_FILE="$COMPOSE_CMD.yml"
    
else
    print_info "Deploying in DEVELOPMENT mode"
    COMPOSE_FILE="$COMPOSE_CMD.yml"
fi

# Pre-deployment checks
print_info "Running pre-deployment checks..."

# Check if ports are available
if netstat -tuln 2>/dev/null | grep -q ":3000 "; then
    print_warning "Port 3000 is already in use. This may cause conflicts."
fi

# Build and deploy
print_info "Building application..."

if [ "$DEPLOYMENT_MODE" = "production" ]; then
    # Production build
    $COMPOSE_CMD -f $COMPOSE_FILE build --no-cache
else
    # Development build
    $COMPOSE_CMD -f $COMPOSE_FILE build
fi

print_status "Application built successfully"

# Stop existing containers
print_info "Stopping existing containers..."
$COMPOSE_CMD -f $COMPOSE_FILE down

# Start services
print_info "Starting services..."
$COMPOSE_CMD -f $COMPOSE_FILE up -d

print_status "Services started successfully"

# Wait for health check
print_info "Waiting for application to be ready..."
sleep 10

# Health check
HEALTH_CHECK_RETRIES=12
HEALTH_CHECK_INTERVAL=5

for i in $(seq 1 $HEALTH_CHECK_RETRIES); do
    if curl -f -s http://localhost:3000/health > /dev/null; then
        print_status "Application is healthy!"
        break
    elif [ $i -eq $HEALTH_CHECK_RETRIES ]; then
        print_error "Application failed to start properly"
        print_info "Checking logs..."
        $COMPOSE_CMD -f $COMPOSE_FILE logs --tail=50
        exit 1
    else
        print_info "Waiting for application... (attempt $i/$HEALTH_CHECK_RETRIES)"
        sleep $HEALTH_CHECK_INTERVAL
    fi
done

# Display deployment information
echo ""
echo "🎉 Deployment Complete!"
echo "======================="
print_status "Memory Memo is running at: http://localhost:3000"
print_info "API Health Check: http://localhost:3000/health"
print_info "API Base URL: http://localhost:3000/api"

if [ "$DEPLOYMENT_MODE" = "production" ]; then
    print_info "Running in PRODUCTION mode"
    print_warning "Make sure to configure proper domain and SSL certificate"
else
    print_info "Running in DEVELOPMENT mode"
fi

echo ""
print_info "Useful Commands:"
echo "  View logs:     $COMPOSE_CMD -f $COMPOSE_FILE logs -f"
echo "  Stop services: $COMPOSE_CMD -f $COMPOSE_FILE down"
echo "  Restart:       ./deploy.sh $DEPLOYMENT_MODE"
echo ""

print_status "Deployment script completed successfully!"