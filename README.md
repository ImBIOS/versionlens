# Zed Version Lens Extension

A Zed extension that displays version information for dependencies in package files.

## Supported Files

- `package.json` - npm/Yarn dependencies
- `Cargo.toml` - Rust dependencies
- `go.mod` - Go modules
- `pyproject.toml` - Python dependencies
- `Gemfile` - Ruby gems
- `pubspec.yaml` - Dart/Flutter packages

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/ImBIOS/zed-versionlens.git
   ```

2. Open Zed and go to Extensions (`Cmd+Shift+P` → "Extensions")

3. Click "Install Dev Extension" and select the `zed-versionlens` directory

## Development

```bash
# Build the extension
cargo build --release

# Copy to Zed extensions directory
cp target/release/libzed_versionlens.so ~/.config/zed/extensions/
```

## License

MIT
