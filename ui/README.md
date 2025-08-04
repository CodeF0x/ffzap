# ffzap UI

ffzap UI is the GUI of the ffzap cli. Available for Windows, Linux and macOS.

## Installation

### Prerequisites

- **FFmpeg**: Must be installed and available in your system PATH

### Download

Download the latest release for your platform from the [releases page](https://github.com/CodeF0x/ffzap/releases/latest).

### Building from Source

```bash
# Clone the repository
git clone https://github.com/CodeF0x/ffzap.git
cd ffzap/ui

# Install dependencies
yarn install

# Build the application
cargo tauri build
```

## Usage

### 1. **Select Input Files**

- **Individual Files**: Click "Browse Files" to select multiple media files
- **File List**: Create a text file with one file path per line, then select it

### 2. **Configure Processing Options**

- **Thread Count**: Set the number of concurrent processing threads (default: 2)
- **FFmpeg Options**: Enter custom FFmpeg parameters (e.g., `-c:v libx265 -preset medium`)
- **Output Pattern**: Define output file naming using placeholders

### 3. **Output Pattern Placeholders**

- `{{name}}` - Original filename without extension
- `{{ext}}` - Original file extension
- `{{dir}}` - Full directory path
- `{{parent}}` - Parent directory name

**Example**: `Output/{{name}}_processed.{{ext}}` → `Output/video_processed.mp4`

### 4. **Advanced Options**

- **Overwrite**: Replace existing output files
- **Verbose**: Show detailed processing logs
- **Delete Source**: Remove original files after successful processing

### 5. **Start Processing**

Click "Start Processing" to begin batch conversion. Monitor progress in real-time and view detailed logs.

## Security Warnings

### Windows SmartScreen

Windows may display a "Windows protected your PC" warning when running ffzap for the first time. This is normal for unsigned applications.

**To run the application:**

1. Click "More info" in the warning dialog
2. Click "Run anyway"
3. The application will run normally

### macOS Gatekeeper

macOS may prevent ffzap from running due to security settings.

**To run the application:**

1. Right-click the ffzap application
2. Select "Open" from the context menu
3. Click "Open" in the confirmation dialog
4. Future launches will work normally

### Why These Warnings Appear

These security mechanisms appear because ffzap is not digitally signed with a code signing certificate. Code signing certificates are expensive (hundreds to thousands of dollars annually) and are typically not cost-effective for small, open-source projects like ffzap.

**This is completely normal and safe** - the warnings are simply the operating system's way of protecting users from potentially malicious software. ffzap is open-source, and you can review the code to verify its safety.

## Development

### Project Structure

```
ui/
├── src/                 # TypeScript source code
│   ├── main.ts         # Main application logic
│   ├── models.ts       # Type definitions
│   ├── dom.ts          # DOM manipulation utilities
│   └── styles.css      # Application styles
├── src-tauri/          # Tauri backend configuration
├── index.html          # Main HTML template
└── package.json        # Node.js dependencies
```

### Development Commands

```bash
# Start development server
cargo tauri dev

# Build for production
cargo tauri build
```

## Troubleshooting

### Common Issues

**FFmpeg not found**

- Ensure FFmpeg is installed and available in your system PATH
- Test by running `ffmpeg -version` in your terminal

**Processing fails**

- Check the verbose logs for detailed error messages
- Verify your FFmpeg options are valid
- Ensure output directory exists and is writable

**Performance issues**

- Reduce thread count if your system is struggling
- Close other resource-intensive applications
- Consider using faster storage (SSD) for input/output files

### Getting Help

- Check the [issues page](https://github.com/your-repo/ffzap/issues) for known problems
- Review the processing logs for specific error messages
- Ensure you're using the latest version of ffzap

## License

This project is licensed under the same license as the main ffzap project. See the main repository for license details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
