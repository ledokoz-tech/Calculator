# Ledokoz OS Calculator

A simple, web-based calculator application built with [Dioxus](https://dioxuslabs.com/), a Rust framework for building user interfaces.

## Features

- **Basic Arithmetic Operations**: Addition, subtraction, multiplication, and division.
- **Square Root Calculation**: Compute the square root of the current display value.
- **Clear Function**: Reset the calculator to its initial state.
- **Responsive UI**: Clean, grid-based layout with styled buttons for a user-friendly experience.
- **Web Deployment**: Runs in the browser using Dioxus Web.

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended).
- A web browser (e.g., Chrome, Firefox).

### Steps
1. Clone the repository:
   ```
   git clone <repository-url>
   cd calculator
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Serve the application:
   ```
   dioxus serve
   ```
   Or, if using the web launcher directly, run:
   ```
   cargo run
   ```

4. Open your browser and navigate to `http://localhost:8080` (or the port specified by Dioxus).

## Usage

- **Entering Numbers**: Click the number buttons (0-9) to input digits.
- **Performing Operations**: Select an operator (+, -, *, /) after entering a number, then enter another number and press "=" to compute.
- **Square Root**: Press "√" to calculate the square root of the current value (only works for non-negative numbers).
- **Clear**: Press "C" to reset the calculator.
- **Display**: The current value or result is shown at the top.

## Building for Android

This application is currently web-based. To deploy on Android devices, you can package it as a Progressive Web App (PWA) or wrap it in a WebView-based app. For native Android builds using Dioxus:

### Prerequisites
- Android SDK and NDK installed.
- Nightly Rust toolchain (since `aarch64-linux-android` target is available in nightly).

### Steps
1. Install nightly Rust and add the target:
   ```
   rustup toolchain install nightly
   rustup target add aarch64-linux-android --toolchain nightly
   ```

2. Switch to Dioxus Mobile (if desired for native app):
   - Update `Cargo.toml` to include `dioxus-mobile` instead of `dioxus-web`.
   - Modify `main.rs` to use `dioxus_mobile::launch` instead of `dioxus_web::launch`.

3. Build for Android:
   ```
   cargo build --target aarch64-linux-android --release
   ```

4. Deploy using Android tools (e.g., via ADB or an APK builder).

For more details, refer to the [Dioxus documentation](https://dioxuslabs.com/docs/mobile).

## Project Structure

- `src/main.rs`: Main application logic and UI components.
- `src/style.css`: Styling for the calculator interface.
- `index.html`: HTML template for the web app.
- `Cargo.toml`: Rust dependencies and project configuration.

## Contributing

Feel free to submit issues or pull requests for improvements, bug fixes, or new features.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.