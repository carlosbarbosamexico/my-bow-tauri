# Bow Desktop

A native desktop application for [Bow](https://bowsapp.com) built with [Tauri](https://tauri.app/).

## Features

- 🚀 **Native Performance** - Built with Rust and Tauri v2
- 🔐 **Secure** - Sandboxed WebView with restricted navigation
- 🔗 **Deep Linking** - Handle `bows://` URLs
- 🔄 **Auto-Updates** - Built-in update mechanism
- 🖥️ **System Tray** - Quick access from the system tray
- 📱 **Cross-Platform** - macOS, Windows, and Linux support

## Prerequisites

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://rustup.rs/) >= 1.70
- Platform-specific dependencies (see below)

### macOS

```bash
xcode-select --install
```

### Windows

- [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (usually pre-installed on Windows 10/11)

### Linux (Debian/Ubuntu)

```bash
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.1-dev \
    libappindicator3-dev \
    librsvg2-dev \
    patchelf \
    libgtk-3-dev
```

## Development

### Install Dependencies

```bash
npm install
```

### Run in Development Mode

```bash
npm run dev
```

This will start the Tauri development server with hot reload.

### Build for Production

```bash
# Build for current platform
npm run build

# Build for specific platforms
npm run build:macos      # Universal macOS (Intel + Apple Silicon)
npm run build:macos-intel # Intel Mac only
npm run build:macos-arm   # Apple Silicon only
npm run build:windows     # Windows x64
npm run build:linux       # Linux x64
```

Build artifacts are output to `src-tauri/target/release/bundle/`.

## Project Structure

```
my-bow-tauri/
├── dist/                    # Frontend files (minimal loader)
│   └── index.html
├── src-tauri/               # Tauri/Rust backend
│   ├── capabilities/        # Permission definitions
│   │   └── default.json
│   ├── icons/              # App icons (all sizes)
│   ├── src/
│   │   ├── lib.rs          # Main app logic
│   │   ├── main.rs         # Entry point
│   │   └── navigation.rs   # URL validation
│   ├── Cargo.toml          # Rust dependencies
│   └── tauri.conf.json     # Tauri configuration
├── .github/
│   └── workflows/
│       └── build.yml       # CI/CD pipeline
├── package.json
└── README.md
```

## Configuration

### App Settings

Edit `src-tauri/tauri.conf.json` to customize:

- App name, version, and identifiers
- Window dimensions and behavior
- Bundle settings (icons, installers)
- Auto-update endpoints

### Security

The app restricts navigation to allowed domains:

- `app.bowsapp.com` - Main application
- `accounts.google.com` - Google OAuth
- `appleid.apple.com` - Apple Sign-In
- `github.com` - GitHub OAuth
- `login.microsoftonline.com` - Microsoft OAuth

Add additional auth providers in `src-tauri/src/lib.rs`.

### Deep Linking

The app handles `bows://` URLs. To test:

```bash
# macOS
open "bows://path/to/resource"

# Linux
xdg-open "bows://path/to/resource"

# Windows
start bows://path/to/resource
```

## Icons

Generate icons from a source image:

```bash
# Using npm script
npm run icon -- /path/to/your/1024x1024-icon.png

# Or generate placeholders (requires ImageMagick)
chmod +x generate-icons.sh
./generate-icons.sh
```

## Auto-Updates

The app supports auto-updates via Tauri's updater plugin.

### Setup

1. Generate an update signing key:
   ```bash
   npx tauri signer generate -w ~/.tauri/myapp.key
   ```

2. Add secrets to GitHub repository:
   - `TAURI_SIGNING_PRIVATE_KEY` - The private key
   - `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` - The key password

3. Update `src-tauri/tauri.conf.json`:
   - Set `plugins.updater.pubkey` to your public key
   - Set `plugins.updater.endpoints` to your update server URLs

### Update Server

You'll need to host update manifests. Example structure:

```
https://releases.bowsapp.com/
├── darwin-x86_64/
│   └── 1.0.0/
│       └── latest.json
├── darwin-aarch64/
│   └── 1.0.0/
│       └── latest.json
├── linux-x86_64/
│   └── 1.0.0/
│       └── latest.json
└── windows-x86_64/
    └── 1.0.0/
        └── latest.json
```

## CI/CD

GitHub Actions automatically builds installers for all platforms:

- **macOS**: `.dmg` and `.app` bundles (Intel + ARM)
- **Windows**: `.msi` and `.exe` (NSIS) installers
- **Linux**: `.deb`, `.rpm`, and `.AppImage`

### Triggers

- Push to `main` branch: Build artifacts (no release)
- Push tag `v*`: Build + create draft release
- Manual dispatch: Option to create release

### Code Signing

Signing is disabled by default. To enable:

1. Add signing certificates to GitHub Secrets
2. Uncomment signing sections in `.github/workflows/build.yml`

#### macOS Signing

- `APPLE_CERTIFICATE` - Base64-encoded .p12 certificate
- `APPLE_CERTIFICATE_PASSWORD` - Certificate password
- `APPLE_SIGNING_IDENTITY` - Signing identity name
- `APPLE_ID` - Apple ID for notarization
- `APPLE_PASSWORD` - App-specific password
- `APPLE_TEAM_ID` - Team ID

#### Windows Signing

- `WINDOWS_CERTIFICATE` - Base64-encoded .pfx certificate
- `WINDOWS_CERTIFICATE_PASSWORD` - Certificate password

## Troubleshooting

### Build fails with WebKit errors (Linux)

Ensure you have the correct WebKit package:

```bash
# Ubuntu 22.04+
sudo apt install libwebkit2gtk-4.1-dev

# Older Ubuntu versions
sudo apt install libwebkit2gtk-4.0-dev
```

### Window doesn't appear (macOS)

Check Console.app for sandboxing errors. Ensure `entitlements` in `tauri.conf.json` are correct.

### Deep links not working

- **macOS**: Rebuild the app after changing `Info.plist`
- **Linux**: Run `xdg-mime default my-bow.desktop x-scheme-handler/bows`
- **Windows**: Reinstall the app to register the protocol handler

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

---

Built with ❤️ using [Tauri](https://tauri.app/)

