#!/bin/bash

# Test script to verify the interactive puzzle image loads correctly
# This script builds the Yew app, serves it, and runs the image test

set -e

echo "🔧 Building Yew app..."
trunk build

echo "🚀 Starting server..."
npx serve dist --listen 8080 &
SERVER_PID=$!

echo "⏳ Waiting for server to start..."
sleep 3

echo "🧪 Running image test..."
npx playwright test tests/visual-puzzle-image.spec.js --config=playwright.config.js

echo "🛑 Stopping server..."
kill $SERVER_PID

echo "✅ Test completed successfully!" 