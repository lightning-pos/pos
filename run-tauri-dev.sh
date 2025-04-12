#!/bin/bash

# Set environment variables for Turso
export TURSO_URL="http://localhost:8080"
export TURSO_TOKEN=""

# Run Tauri dev
pnpm tauri dev
