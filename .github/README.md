# Ledokoz Calculator

Ledokoz Calculator is a modern, lightweight calculator application built with Rust and the Dioxus framework. This cross-platform application provides basic arithmetic operations with an elegant, responsive user interface that works across multiple platforms.

## Features

- ✨ **Basic Operations**: Addition, subtraction, multiplication, and division
- 🧮 **Advanced Functions**: Square root calculations
- 🧹 **Clear Function**: Reset the calculator state with the C button
- 📱 **Cross-Platform**: Works on Windows, macOS, Linux, and Android
- 🌐 **Web Support**: Runs directly in browsers using WebAssembly
- 📐 **Responsive Design**: Adapts to various screen sizes
- 🔧 **Modern Tech Stack**: Built with Rust and Dioxus for safety and performance

## Technology Stack

- **Frontend Framework**: [Dioxus](https://dioxuslabs.com/) (Rust-based declarative GUI)
- **Build System**: Cargo with Github Actions for continuous integration and deployment
- **UI Rendering**: WebAssembly for web deployment, native rendering for desktop/mobile
- **State Management**: Dioxus Signals
- **Packaging**: Tauri for native applications for Windows, macOS, Linux, and Android.

## Platform Support

The calculator app is designed to work across multiple platforms:

- **Web**: Deployed as WebAssembly (WASM) for browser usage
- **Desktop**: Available for Windows (.exe), macOS (.dmg), and Linux (.deb/.AppImage)
- **Mobile**: Android support (.apk) with plans for iOS

## Installation

### Web Version

Simply visit the deployed URL to use the calculator directly in your browser.

### Desktop Versions

Download the appropriate installer from the [Releases](https://github.com/ledokoz-tech/calculator/releases) page:

- **Windows**: Download the `.exe` or `.msi` file
- **macOS**: Download the `.dmg` file
- **Linux**: Choose between `.deb` package or `.AppImage` portable version

### Mobile Version

- **Android**: Download the `.apk` file from the releases page

## Building from Source

### Prerequisites

- Rust (latest stable version recommended)
- Cargo
- Node.js (for Dioxus CLI)

### Setup

1. Install Rust:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rust-lang.org/install.sh | sh
   ```

2. Install the Dioxus CLI:

   ```bash
   cargo install dioxus-cli
   ```

3. Clone the repository:

   ```bash
   git clone https://github.com/ledokoz-tech/calculator.git
   cd calculator
   ```

### Running in Development Mode

To run the calculator in development mode:

```bash
dioxus serve
```

Then open your browser to http://localhost:8080

### Building for Production

#### Web Build

```bash
dioxus build --release
```

#### Desktop Builds

For your current platform:

```bash
cargo tauri build
```

For specific platforms:

- **Windows**: `cargo tauri build --target x86_64-pc-windows-msvc`
- **macOS**: `cargo tauri build --target x86_64-apple-darwin`
- **Linux**: `cargo tauri build --target x86_64-unknown-linux-gnu`

#### Android Build

```bash
# Requires Android SDK/NDK setup
cargo tauri android build
```

## Architecture

The calculator application follows a component-based architecture using Dioxus:

- **Components**: Organized using Dioxus components for UI elements like buttons and display
- **State Management**: Utilizes Dioxus Signals for reactive state management
- **Event Handling**: Handles user interactions through event callbacks
- **Styling**: CSS for styling with responsive design principles

### Key Components

- `App`: Main application component
- `Calculator`: Core calculator logic component
- `CalcButton`: Reusable button component
- `Display`: Shows current input and results

## Contributing

We welcome contributions to the Ledokoz OS Calculator project!

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Commit your changes (`git commit -m 'Add amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

### Development Guidelines

- Write clean, readable Rust code following community standards
- Test changes thoroughly across different platforms when possible
- Update documentation as needed
- Follow the existing code style and patterns

## Project Structure

```
calculator/
├── assets/                 # Static assets like stylesheets
│   └── styles.css          # Calculator-specific styles
├── src/                    # Dioxus source files
│   ├── app.rs              # Main application logic
│   └── main.rs             # Entry point
├── src-tauri/              # Tauri configuration for native apps
│   ├── src/
│   ├── Cargo.toml
│   ├── tauri.conf.json     # Tauri configuration
│   └── ...
├── .github/                # GitHub-specific files
│   └── README.md           # This file
├── .gitignore
├── Cargo.toml              # Rust dependencies
├── Dioxus.toml             # Dioxus configuration
├── README.md               # Top-level project info
└── ...
```

## Automated Builds

The project includes GitHub Actions workflows for automated building and releasing:

- **Continuous Integration**: Automatically builds and tests changes
- **Release Pipeline**: Creates distributable packages for all supported platforms
- **Cross-Platform Support**: Builds for Windows, macOS, Linux, and Android from the same codebase

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## Support

If you encounter issues or have questions:

1. Check the existing [Issues](https://github.com/ledokoz-tech/calculator/issues)
2. Create a new issue if needed, providing as much detail as possible
3. Include information about your platform and steps to reproduce any bugs
