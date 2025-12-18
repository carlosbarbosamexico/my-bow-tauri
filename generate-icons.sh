#!/bin/bash
# Generate placeholder icons for development
# Replace these with actual icons before release

set -e

ICONS_DIR="src-tauri/icons"
mkdir -p "$ICONS_DIR"

# Check if ImageMagick is available
if ! command -v convert &> /dev/null; then
    echo "ImageMagick not found. Creating minimal placeholder files..."
    
    # Create minimal 1x1 transparent PNGs as placeholders
    # These are valid PNG files but should be replaced with actual icons
    
    # Minimal transparent PNG (1x1)
    PLACEHOLDER_PNG='\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x06\x00\x00\x00\x1f\x15\xc4\x89\x00\x00\x00\nIDATx\x9cc\x00\x01\x00\x00\x05\x00\x01\r\n-\xb4\x00\x00\x00\x00IEND\xaeB`\x82'
    
    echo "Creating placeholder PNG files..."
    touch "$ICONS_DIR/icon.png"
    touch "$ICONS_DIR/32x32.png"
    touch "$ICONS_DIR/128x128.png"
    touch "$ICONS_DIR/128x128@2x.png"
    touch "$ICONS_DIR/icon.icns"
    touch "$ICONS_DIR/icon.ico"
    
    echo ""
    echo "⚠️  Placeholder icon files created."
    echo "   These must be replaced with actual icons before building."
    echo ""
    echo "   Generate icons using:"
    echo "   npx tauri icon /path/to/your/1024x1024-icon.png"
    
    exit 0
fi

# If ImageMagick is available, create proper placeholder icons

echo "Creating placeholder icons with ImageMagick..."

# Create a simple gradient icon with "B" letter
convert -size 1024x1024 \
    -define gradient:angle=135 \
    gradient:'#6366f1-#8b5cf6' \
    -gravity center \
    -fill white \
    -font Helvetica-Bold \
    -pointsize 600 \
    -annotate 0 'B' \
    "$ICONS_DIR/icon-source.png"

# Generate all sizes
convert "$ICONS_DIR/icon-source.png" -resize 512x512 "$ICONS_DIR/icon.png"
convert "$ICONS_DIR/icon-source.png" -resize 32x32 "$ICONS_DIR/32x32.png"
convert "$ICONS_DIR/icon-source.png" -resize 128x128 "$ICONS_DIR/128x128.png"
convert "$ICONS_DIR/icon-source.png" -resize 256x256 "$ICONS_DIR/128x128@2x.png"

# Create .icns for macOS (requires iconutil on macOS)
if [[ "$OSTYPE" == "darwin"* ]]; then
    ICONSET="$ICONS_DIR/icon.iconset"
    mkdir -p "$ICONSET"
    
    convert "$ICONS_DIR/icon-source.png" -resize 16x16 "$ICONSET/icon_16x16.png"
    convert "$ICONS_DIR/icon-source.png" -resize 32x32 "$ICONSET/icon_16x16@2x.png"
    convert "$ICONS_DIR/icon-source.png" -resize 32x32 "$ICONSET/icon_32x32.png"
    convert "$ICONS_DIR/icon-source.png" -resize 64x64 "$ICONSET/icon_32x32@2x.png"
    convert "$ICONS_DIR/icon-source.png" -resize 128x128 "$ICONSET/icon_128x128.png"
    convert "$ICONS_DIR/icon-source.png" -resize 256x256 "$ICONSET/icon_128x128@2x.png"
    convert "$ICONS_DIR/icon-source.png" -resize 256x256 "$ICONSET/icon_256x256.png"
    convert "$ICONS_DIR/icon-source.png" -resize 512x512 "$ICONSET/icon_256x256@2x.png"
    convert "$ICONS_DIR/icon-source.png" -resize 512x512 "$ICONSET/icon_512x512.png"
    convert "$ICONS_DIR/icon-source.png" -resize 1024x1024 "$ICONSET/icon_512x512@2x.png"
    
    iconutil -c icns "$ICONSET" -o "$ICONS_DIR/icon.icns"
    rm -rf "$ICONSET"
else
    # Create a placeholder .icns (will need to be regenerated on macOS)
    cp "$ICONS_DIR/icon.png" "$ICONS_DIR/icon.icns"
fi

# Create .ico for Windows
convert "$ICONS_DIR/icon-source.png" \
    -define icon:auto-resize=256,128,96,64,48,32,16 \
    "$ICONS_DIR/icon.ico"

# Cleanup
rm -f "$ICONS_DIR/icon-source.png"

echo "✅ Icons generated successfully!"
echo ""
echo "Files created:"
ls -la "$ICONS_DIR"

