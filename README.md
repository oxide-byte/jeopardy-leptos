# Jeopardy Leptos

A modern, responsive Jeopardy! quiz show web application built with [Leptos](https://leptos.dev/) (Rust + WebAssembly) and [Tailwind CSS](https://tailwindcss.com/).

Deployed under GitHub Pages: https://oxide-byte.github.io/jeopardy-leptos/

> **Note**: This project was generated with [JetBrains Air](https://www.jetbrains.com/air/) and Gemini 3.7 Flash.

## Features

- **Classic Game Board**: Six categories with ascending clue values ($200 to $1000).
- **Interactive Clues & Scoring**: Modal clue presentation, response reveal mechanism, and real-time score tracking.
- **Splash Screen & Rules**: Welcoming landing interface explaining game rules and guidelines.
- **Fast WebAssembly Runtime**: Powered by Leptos CSR (Client-Side Rendering) for high-performance UI updates.
- **Modern Responsive Design**: Custom Tailwind CSS theme tailored for desktop and mobile displays.

## Prerequisites

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/) (nightly or latest stable with WebAssembly target):
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- [Trunk](https://trunkrs.dev/) (Wasm web application bundler for Rust):
  ```bash
  cargo install trunk
  ```

## Getting Started

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd jeopardy-leptos
   ```

2. **Run development server**:
   ```bash
   trunk serve
   ```
   Open your browser at `http://127.0.0.1:8080`.

3. **Build for production**:
   ```bash
   trunk build --release
   ```
   The compiled static assets will be located in the `dist/` directory.

4. Build for GitHub Pages:

```bash
trunk build --release --public-url="/jeopardy-leptos/"
```

- Delete old files from docs.
- Copy manually the files from dist to docs.

## Project Structure

```text
├── assets/             # Static assets (fonts, icons)
├── docs/               # Source for GitHub Pages (manual Copy/Past of dist)
├── public/             # Public static files
├── src/
│   ├── components/     # Modular Leptos UI components
│   │   ├── board/      # Game board, clues, score header, and modal
│   │   ├── splash/     # Splash screen, hero, rules modal, and footer
│   │   ├── app.rs      # Root application component
│   │   └── mod.rs      # Module exports
│   └── main.rs         # Application entry point
├── Cargo.toml          # Rust dependencies & crate configuration
├── index.html          # HTML entry point configured for Trunk
├── tailwind.css        # Tailwind styling & custom utility classes
└── tailwind.config.js  # Tailwind configuration
```

## License

This project is licensed under the [MIT License](LICENSE).
