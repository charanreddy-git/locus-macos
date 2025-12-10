#!/bin/bash

# Install script for Locus on macOS
# This script installs Locus and sets up the necessary permissions

set -e

APP_NAME="Locus"
BUNDLE_ID="com.sushant.locus"
INSTALL_DIR="/Applications"
PLIST_DIR="$HOME/Library/LaunchAgents"
PLIST_FILE="$PLIST_DIR/$BUNDLE_ID.plist"

echo "🎯 Installing Locus for macOS..."

# Check if we're on macOS
if [[ "$(uname)" != "Darwin" ]]; then
    echo "❌ This installer is for macOS only"
    exit 1
fi

# Check if we're on Apple Silicon
if [[ "$(uname -m)" != "arm64" ]]; then
    echo "❌ This installer is for Apple Silicon Macs only"
    exit 1
fi

# Check if app bundle exists
if [[ ! -d "$APP_NAME.app" ]]; then
    echo "❌ $APP_NAME.app not found in current directory"
    echo "   Please run this script from the directory containing $APP_NAME.app"
    exit 1
fi

# Install the app
echo "📦 Installing $APP_NAME to $INSTALL_DIR..."
sudo cp -R "$APP_NAME.app" "$INSTALL_DIR/"
sudo chown -R root:wheel "$INSTALL_DIR/$APP_NAME.app"

# Set up launchd service (optional)
read -p "🤖 Would you like to set up automatic startup? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "⚙️  Setting up automatic startup..."
    
    # Create LaunchAgents directory if it doesn't exist
    mkdir -p "$PLIST_DIR"
    
    # Create the plist file
    cat > "$PLIST_FILE" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>$BUNDLE_ID</string>
    
    <key>ProgramArguments</key>
    <array>
        <string>$INSTALL_DIR/$APP_NAME.app/Contents/MacOS/locus</string>
    </array>
    
    <key>RunAtLoad</key>
    <true/>
    
    <key>KeepAlive</key>
    <true/>
    
    <key>StandardErrorPath</key>
    <string>/tmp/locus.err</string>
    
    <key>StandardOutPath</key>
    <string>/tmp/locus.out</string>
    
    <key>WorkingDirectory</key>
    <string>$INSTALL_DIR/$APP_NAME.app/Contents/MacOS</string>
</dict>
</plist>
EOF
    
    # Load the service
    launchctl load "$PLIST_FILE" 2>/dev/null || true
    echo "✅ Automatic startup configured"
fi

echo "🎉 Installation complete!"
echo ""
echo "📝 Next steps:"
echo "   1. Open $APP_NAME from Applications or Spotlight"
echo "   2. Grant Screen Recording permission when prompted"
echo "   3. Enjoy tracking your productivity!"
echo ""
echo "📍 Data will be stored in: ~/Library/Application Support/Locus"
echo ""
echo "🔧 To uninstall later:"
echo "   - Remove /Applications/$APP_NAME.app"
echo "   - Remove $PLIST_FILE (if created)"
echo "   - Remove ~/Library/Application Support/Locus"