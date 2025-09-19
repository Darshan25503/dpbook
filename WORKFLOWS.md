# GitHub Workflows Documentation

This document describes the GitHub Actions workflows configured for the dpbook project.

## Workflows Overview

### 1. CI/CD Pipeline (`.github/workflows/ci.yml`)

**Triggers:**
- Push to `main` or `develop` branches
- Pull requests to `main` branch
- Release events

**Jobs:**

#### Test Suite
- Runs on Ubuntu with Rust stable, beta, and nightly
- Performs code formatting checks (`cargo fmt`)
- Runs linting with Clippy (`cargo clippy`)
- Executes all unit and integration tests
- Generates documentation

#### Security Audit
- Runs `cargo audit` to check for security vulnerabilities
- Fails the build if critical vulnerabilities are found

#### Build
- Cross-compiles for multiple platforms:
  - Linux (x86_64-unknown-linux-gnu)
  - Windows (x86_64-pc-windows-msvc)
  - macOS Intel (x86_64-apple-darwin)
  - macOS Apple Silicon (aarch64-apple-darwin)
- Uploads build artifacts for each platform

#### Code Coverage
- Generates code coverage reports using `cargo-llvm-cov`
- Uploads coverage to Codecov

#### Docker Build
- Builds multi-platform Docker images (amd64, arm64)
- Pushes to Docker Hub on releases

### 2. Release Workflow (`.github/workflows/release.yml`)

**Triggers:**
- Push of version tags (e.g., `v0.1.2`)

**Jobs:**

#### Create Release
- Creates a GitHub release with changelog template
- Provides download links and installation instructions

#### Build and Upload
- Cross-compiles for all supported platforms
- Creates compressed archives (tar.gz for Unix, zip for Windows)
- Uploads release assets to GitHub

#### Publish to crates.io
- Publishes the package to the Rust package registry
- Requires `CRATES_IO_TOKEN` secret

#### Publish Docker Image
- Builds and pushes Docker images with version tags
- Tags both with version number and `latest`

### 3. Dependabot Auto-merge (`.github/workflows/dependabot-auto-merge.yml`)

**Triggers:**
- Dependabot pull requests

**Functionality:**
- Automatically merges Dependabot PRs after CI passes
- Uses squash merge strategy
- Only applies to dependency updates

## Required Secrets

To use these workflows, configure the following secrets in your GitHub repository:

### Required for Release Workflow
- `CRATES_IO_TOKEN`: Token for publishing to crates.io
- `DOCKER_USERNAME`: Docker Hub username
- `DOCKER_PASSWORD`: Docker Hub password or access token

### Optional for Enhanced Features
- `CODECOV_TOKEN`: Token for Codecov integration (optional, works without)

## Setting Up Secrets

1. Go to your repository on GitHub
2. Navigate to Settings → Secrets and variables → Actions
3. Click "New repository secret"
4. Add each required secret

### Getting Tokens

#### Crates.io Token
1. Visit [crates.io](https://crates.io/)
2. Log in with your GitHub account
3. Go to Account Settings → API Tokens
4. Create a new token with publish permissions

#### Docker Hub Token
1. Visit [Docker Hub](https://hub.docker.com/)
2. Go to Account Settings → Security
3. Create a new access token

## Usage Instructions

### Continuous Integration
The CI workflow runs automatically on every push and pull request. It will:
- Validate code formatting and style
- Run all tests
- Check for security vulnerabilities
- Build for all platforms

### Creating a Release

1. **Update version in Cargo.toml**
   ```toml
   [package]
   version = "0.1.3"  # Increment version
   ```

2. **Commit and push changes**
   ```bash
   git add Cargo.toml
   git commit -m "Bump version to 0.1.3"
   git push origin main
   ```

3. **Create and push a tag**
   ```bash
   git tag v0.1.3
   git push origin v0.1.3
   ```

4. **The release workflow will automatically:**
   - Create a GitHub release
   - Build binaries for all platforms
   - Publish to crates.io
   - Push Docker images

### Manual Docker Build

To build Docker images locally:

```bash
# Build for current platform
docker build -t dpbook:local .

# Build for multiple platforms (requires buildx)
docker buildx build --platform linux/amd64,linux/arm64 -t dpbook:multi .
```

### Local Testing

Before pushing, you can run the same checks locally:

```bash
# Format check
cargo fmt --check

# Linting
cargo clippy --all-targets --all-features -- -D warnings

# Tests
cargo test --all-features

# Security audit
cargo install cargo-audit
cargo audit

# Build for release
cargo build --release
```

## Workflow Status

You can monitor workflow status through:
- GitHub Actions tab in your repository
- Status badges in README.md
- Email notifications (configurable in GitHub settings)

## Troubleshooting

### Common Issues

1. **Failed crates.io publish**
   - Check if version already exists
   - Verify CRATES_IO_TOKEN is valid
   - Ensure all required fields in Cargo.toml

2. **Docker build failures**
   - Check Dockerfile syntax
   - Verify base image availability
   - Check Docker Hub credentials

3. **Test failures**
   - Run tests locally first
   - Check for platform-specific issues
   - Verify all dependencies are available

### Getting Help

- Check workflow logs in GitHub Actions tab
- Review error messages carefully
- Consult GitHub Actions documentation
- Open an issue if problems persist

## Customization

### Adding New Platforms

To add support for new platforms, update the build matrix in both CI and release workflows:

```yaml
matrix:
  include:
    - os: ubuntu-latest
      target: aarch64-unknown-linux-gnu  # New target
      artifact_name: dpbook
      asset_name: dpbook-linux-aarch64.tar.gz
```

### Modifying Release Notes

Edit the release body template in `.github/workflows/release.yml`:

```yaml
body: |
  ## Your custom release notes template
  
  ### Changes
  - List changes here
```

### Adding New Checks

Add new jobs to the CI workflow:

```yaml
new-check:
  name: New Check
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: Run new check
      run: your-command-here
```
