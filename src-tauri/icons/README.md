# Application Icons

This directory should contain the application icons in various formats and sizes.

## Required Icons

| File | Size | Platform |
|------|------|----------|
| `icon.png` | 512x512 | Source icon for tray |
| `32x32.png` | 32x32 | Windows/Linux |
| `128x128.png` | 128x128 | All platforms |
| `128x128@2x.png` | 256x256 | macOS Retina |
| `icon.icns` | Multi-size | macOS |
| `icon.ico` | Multi-size | Windows |

## Generating Icons

You can use the Tauri CLI to generate all icon variants from a single source:

```bash
# From the project root
npm run icon -- /path/to/your/icon.png

# Or directly with the Tauri CLI
npx tauri icon /path/to/your/1024x1024-icon.png
```

The source icon should be:
- At least 1024x1024 pixels
- PNG format with transparency
- Square aspect ratio

## Placeholder Note

The icons in this directory are placeholders. Replace them with your actual app icons before release.

