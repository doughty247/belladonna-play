#!/bin/bash

echo "======================================"
echo "   Belladonna Play SDK Demo Runner   "
echo "======================================"
echo ""

# Check if we're in the SDK directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Please run this script from the sdk/ directory"
    exit 1
fi

echo "Available demos:"
echo "1. Interactive Demo System"
echo "2. Basic Integration Example"  
echo "3. License Test Example"
echo ""

read -p "Select demo to run (1-3): " choice

case $choice in
    1)
        echo "Starting Interactive Demo System..."
        echo ""
        cargo run --example interactive_demo --features ffi
        ;;
    2)
        echo "Running Basic Integration Example..."
        echo ""
        cargo run --example basic_integration --features ffi
        ;;
    3)
        echo "Running License Test Example..."
        echo ""
        cargo run --example license_test --features ffi,license-gui
        ;;
    *)
        echo "Invalid selection"
        exit 1
        ;;
esac