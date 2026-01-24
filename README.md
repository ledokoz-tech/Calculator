# Calculator

## Installation and Setup

This is a cross-platform calculator application built with Dioxus UI framework and Tauri. It runs on both web and native platforms.

### Prerequisites

- Rust (latest stable version)
- Node.js and npm (for Dioxus CLI)

### Setup Instructions

1. Install the Dioxus CLI:

   ```bash
   cargo install dioxus-cli
   ```

2. Make sure you have the correct version of Tauri CLI installed that matches your project's Tauri version (currently using Tauri 2.0.0-rc.0):

   ```bash
   cargo install tauri-cli --version 2.0.0-rc.0
   ```

   Alternatively, you can use `cargo tauri` directly without installing the CLI separately.

3. Install dependencies:

   ```bash
   cd calculator
   dx install
   ```

### Running the Application

#### For Desktop

Run the application in development mode:

```bash
cargo tauri dev
```

Or if you prefer to use the Tauri CLI directly:

```bash
cargo tauri dev
```

#### For Web

To run the application in a web browser:

```bash
dx serve
```

Then visit `http://localhost:1420` in your browser.

### Building the Application

To create a production build for desktop:

```bash
cargo tauri build
```

For a web production build:

```bash
dx build --release
```

## Troubleshooting

If you encounter compilation errors related to Tauri CLI, particularly errors about unresolved imports like `AppUrl` and `WindowUrl`, make sure you have the correct version of the Tauri CLI installed that matches your project's Tauri version. In this project's case, use:

```bash
cargo install tauri-cli --version 2.0.0-rc.0
```

This addresses the API mismatches that can occur when using different versions of the Tauri framework and CLI tool.
