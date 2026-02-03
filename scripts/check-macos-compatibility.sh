#!/bin/bash

echo "=========================================="
echo "QuantHUD macOS Compatibility Check"
echo "=========================================="
echo ""

# Check macOS version
echo "1. macOS Version:"
sw_vers
echo ""

# Check architecture
echo "2. CPU Architecture:"
uname -m
echo ""

# Check if it's Intel or Apple Silicon
if [[ $(uname -m) == 'arm64' ]]; then
    echo "   → Apple Silicon (M1/M2/M3/M4)"
elif [[ $(uname -m) == 'x86_64' ]]; then
    echo "   → Intel (x64)"
else
    echo "   → Unknown architecture"
fi
echo ""

# Check macOS version number
OS_VERSION=$(sw_vers -productVersion)
MAJOR_VERSION=$(echo $OS_VERSION | cut -d. -f1)
MINOR_VERSION=$(echo $OS_VERSION | cut -d. -f2)

echo "3. Compatibility Check:"
if [ "$MAJOR_VERSION" -ge 11 ]; then
    echo "   ✅ macOS $OS_VERSION is compatible (11.0+)"
elif [ "$MAJOR_VERSION" -eq 10 ] && [ "$MINOR_VERSION" -ge 13 ]; then
    echo "   ✅ macOS $OS_VERSION is compatible (10.13+)"
else
    echo "   ❌ macOS $OS_VERSION is TOO OLD (need 10.13+)"
    echo "   → Please update macOS or use a newer Mac"
fi
echo ""

# Check if Rosetta 2 is installed (for Apple Silicon)
if [[ $(uname -m) == 'arm64' ]]; then
    echo "4. Rosetta 2 Check (for Intel apps on Apple Silicon):"
    if /usr/bin/pgrep -q oahd; then
        echo "   ✅ Rosetta 2 is installed"
    else
        echo "   ⚠️  Rosetta 2 might not be installed"
        echo "   → Install with: softwareupdate --install-rosetta"
    fi
    echo ""
fi

# Check Gatekeeper status
echo "5. Gatekeeper Status:"
spctl --status
echo ""

echo "=========================================="
echo "Please send this output to the developer"
echo "=========================================="

