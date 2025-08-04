# ffzap ⚡

A multithreaded media processing toolkit built with Rust. ffzap provides both command-line and graphical interfaces for batch processing media files using FFmpeg. If FFmpeg can do it, ffzap can do it - as many files in parallel as your system can handle.

## What is ffzap?

ffzap is a high-performance media processing solution that leverages FFmpeg's capabilities with modern Rust performance. It's designed for users who need to process large batches of media files efficiently, whether they prefer command-line tools or graphical interfaces.

**Key Features:**

- ⚡ **Multithreaded Processing**: Process multiple files simultaneously
- 🎯 **FFmpeg Power**: Full access to all FFmpeg capabilities
- 🖥️ **Cross-platform**: Windows, macOS, and Linux support
- 🎨 **Dual Interface**: Both CLI and GUI options
- 🔧 **Flexible Output**: Customizable file naming with placeholders
- 📊 **Real-time Progress**: Live progress tracking and detailed logging

## Project Components

ffzap is organized into three main components:

### 🔧 **Shared Core (`core/`)**

The heart of ffzap - a Rust library that provides the core processing functionality.

**What it does:**

- Handles all FFmpeg operations and file processing
- Manages multithreaded execution
- Provides progress tracking and logging
- Defines the command-line argument structure
- Offers a clean API for both CLI and GUI applications

**Key Features:**

- `Processor`: Core processing engine with thread management
- `Logger`: Comprehensive logging system with file output
- `Progress`: Real-time progress tracking with ETA support
- `CmdArgs`: Structured command-line argument handling
- `load_paths`: Utility for loading file paths from various sources

**Usage Example:**

```rust
use ffzap_core::{CmdArgs, Processor, Logger, Progress, load_paths};
use std::sync::Arc;

// Create processing configuration
let cmd_args = CmdArgs {
    thread_count: 2,
    ffmpeg_options: Some("-c:v libx265 -preset medium".to_string()),
    input: Some(vec!["video1.mp4".to_string(), "video2.mp4".to_string()]),
    output: "output/{{name}}_processed.{{ext}}".to_string(),
    // ... other options
};

// Set up processing pipeline
let paths = load_paths(&cmd_args);
let progress = Arc::new(Progress::new(paths.len(), false));
let logger = Arc::new(Logger::new(progress.clone()));
let processor = Processor::new(logger, progress);

// Process files
processor.process_files(/* ... */);
```

Read more about its purpose and usage [here](/core/README.md).

### 💻 **Command Line Interface (`cli/`)**

A fast, efficient command-line tool for power users and automation.

**What it does:**

- Provides a familiar CLI experience similar to FFmpeg
- Handles command-line argument parsing with clap
- Offers batch processing capabilities
- Includes comprehensive help and documentation

**Key Features:**

- Intuitive command-line syntax
- Support for file lists and wildcard patterns
- Real-time progress bars and verbose logging
- Cross-platform binary distribution
- Package manager support (Homebrew, Winget, Cargo)

**Usage Examples:**

```bash
# Basic usage
ffzap -i video1.mp4 video2.mp4 -f "-c:v libx265" -o "{{name}}_encoded.{{ext}}" -t 1

# Batch processing with multiple threads
ffzap --file-list videos.txt -f "-c:v libx265 -preset medium" -o "output/{{name}}.mp4" -t 4

# Complex FFmpeg operations
ffzap -i *.mp4 -f "-vf scale=1920:1080 -c:v libx264 -c:a aac" -o "hd/{{name}}.mp4" -t 2
```

### 🖥️ **Graphical User Interface (`ui/`)**

A modern, cross-platform desktop application built with Tauri and TypeScript.

**What it does:**

- Provides an intuitive graphical interface for ffzap
- Offers drag-and-drop file selection
- Includes real-time progress visualization
- Features comprehensive logging display

**Key Features:**

- Modern, responsive web-based UI
- Tabbed interface for different input methods
- Real-time progress bars and status updates
- Advanced options configuration
- Cross-platform desktop application

Read more about its features and local development [here](/ui/README.md).

## Architecture Overview

```
┌─────────────────┐    ┌─────────────────┐
│   CLI (cli/)    │    │   GUI (ui/)     │
│                 │    │                 │
│ • CLI Wrapper   │    │ • GUI Wrapper   │
└─────────┬───────┘    └─────────┬───────┘
          │                      │
          └──────────┬───────────┘
                     │
          ┌──────────▼──────────┐
          │   Core (core/)      │
          │                     │
          │ • Processor         │
          │ • Logger            │
          │ • Progress          │
          │ • CmdArgs           │
          │ • Utilities         │
          └──────────┬──────────┘
                     │
          ┌──────────▼───────────┐
          │   FFmpeg Engine      │
          │ (External Dependency)│
          └──────────────────────┘
```

## Getting Started

### Quick Start

1. **Install FFmpeg** on your system
2. **Choose your interface**:
   - **CLI**: Install via `cargo install ffzap` or download from releases
   - **GUI**: Download the latest release for your platform
3. **Start processing** your media files!

### Installation Options

**CLI Installation:**

```bash
# Via Cargo
cargo install ffzap

# Via Homebrew (macOS/Linux)
brew tap CodeF0x/formulae
brew install ffzap

# Via Winget (Windows)
winget install CodeF0x.ffzap
```

**GUI Installation:**
Download the latest release from [GitHub Releases](https://github.com/CodeF0x/ffzap/releases/latest)

## Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/CodeF0x/ffzap.git
cd ffzap

# Build CLI
cargo build --release -p ffzap

# Build GUI
cargo tauri build
```

### Project Structure

```
ffzap/
├── core/            # Core processing library
│   ├── src/         # Rust source code
│   └── Cargo.toml   # Library dependencies
├── cli/             # Command-line interface
│   ├── src/         # CLI implementation
│   └── Cargo.toml   # CLI dependencies
├── ui/              # Graphical user interface
│   ├── src/         # TypeScript frontend
│   ├── src-tauri/   # Tauri backend
│   └── package.json # Node.js dependencies
└── Cargo.toml       # Workspace configuration
```

## Performance

ffzap is designed for maximum performance:

- **Multithreaded**: Process multiple files simultaneously
- **Efficient**: Minimal overhead over direct FFmpeg usage
- **Scalable**: Adjust thread count based on your system capabilities
- **Optimized**: Rust-based backend for maximum speed

## License

This project uses a custom license that allows:

- ✅ **Use**: Any purpose
- ✅ **Modify**: As you like
- ✅ **Distribute**: With attribution to original author
- ✅ **Sell**: Substantially modified versions only

**Restrictions:**

- ❌ No selling of original or minimally modified versions
- ❌ Must credit original author (Tobias "CodeF0x" Oettl) for unmodified distributions

## Contributing

Contributions are welcome! Please feel free to submit Pull Requests or open issues for bugs and feature requests.

## Support

- **Documentation**: Check the individual component READMEs
- **Issues**: [GitHub Issues](https://github.com/CodeF0x/ffzap/issues)
