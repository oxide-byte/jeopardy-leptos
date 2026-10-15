default:
    @just --list

# Run development server
serve:
    trunk serve

# Build for production
build:
    trunk build --release

# Build for GitHub Pages and copy to docs/
build-pages:
    rm -rf docs
    mkdir -p docs
    trunk build --release --public-url="/jeopardy-leptos/"
    cp -r dist/. docs/

# Alias for build-pages
pages: build-pages
build-github-pages: build-pages

# Clean build artifacts
clean:
    rm -rf dist docs
    cargo clean
