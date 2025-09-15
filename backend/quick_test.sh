#!/bin/bash

echo "🧪 Quick Integration Test"

# Start server in background
export DATABASE_URL="sqlite://quick_test.db"
export PORT="3002"
./target/debug/memory_memo &
SERVER_PID=$!

sleep 3

# Test health
curl -s http://127.0.0.1:3002/health
echo ""

# Test registration
echo "Testing registration..."
curl -s -X POST http://127.0.0.1:3002/api/auth/register \
  -H "Content-Type: application/json" \
  -c cookies.txt \
  -d '{"username":"testuser","password":"testpass123"}' | jq
echo ""

# Test login
echo "Testing login..."
curl -s -X POST http://127.0.0.1:3002/api/auth/login \
  -H "Content-Type: application/json" \
  -b cookies.txt -c cookies.txt \
  -d '{"username":"testuser","password":"testpass123"}' | jq
echo ""

# Test memo creation
echo "Testing memo creation..."
curl -s -X POST http://127.0.0.1:3002/api/memos/forever \
  -H "Content-Type: application/json" \
  -b cookies.txt \
  -d '{"content":"Test memo"}' | jq
echo ""

# Test memo listing
echo "Testing memo listing..."
curl -s -X GET http://127.0.0.1:3002/api/memos \
  -b cookies.txt | jq
echo ""

# Cleanup
kill $SERVER_PID 2>/dev/null || true
rm -f cookies.txt quick_test.db*

echo "🎉 Quick test complete"