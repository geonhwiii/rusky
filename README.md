# 🐺 rusky

**Simple Git hooks manager written in Rust** - A dependency-free alternative to husky

[![npm version](https://badge.fury.io/js/@gunw.dan/rusky.svg)](https://badge.fury.io/js/@gunw.dan/rusky)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## ✨ Features

- 🚀 **Drop-in Replacement**: Works exactly like husky - no learning curve
- 🔧 **Simple Setup**: One command to get started
- 📦 **Zero Runtime Dependencies**: No Node.js dependencies to manage
- 🎯 **Git Integration**: Seamless integration with Git hooks
- 🛡️ **Type Safe**: Built with Rust's compile-time safety guarantees
- 🌍 **Cross Platform**: Works on macOS, Linux, and Windows

## 📦 Installation

```bash
npm install --save-dev @gunw.dan/rusky
```

That's it! No additional dependencies, no complex setup. rusky works exactly like husky but with zero runtime dependencies.

## 🚀 Quick Start

1. Initialize rusky in your project:
```bash
npx @gunw.dan/rusky init
```

2. Add a git hook:
```bash
npx @gunw.dan/rusky add pre-commit "npm test"
```

3. Add more hooks as needed:
```bash
npx @gunw.dan/rusky add pre-push "npm run lint"
npx @gunw.dan/rusky add commit-msg "npx commitlint --edit $1"
```

## 📚 Commands

### `rusky init`
Initialize rusky in your project. This creates a `.rusky` directory and sets up the git hooks directory.

```bash
npx @gunw.dan/rusky init
```

### `rusky add <hook> <command>`
Add a git hook with the specified command.

```bash
npx @gunw.dan/rusky add pre-commit "npm test"
npx @gunw.dan/rusky add pre-push "npm run build"
```

### `rusky remove <hook>`
Remove a git hook.

```bash
npx @gunw.dan/rusky remove pre-commit
```

### `rusky list`
List all configured hooks.

```bash
npx @gunw.dan/rusky list
```

### `rusky install`
Install all configured git hooks.

```bash
npx @gunw.dan/rusky install
```

### `rusky uninstall`
Uninstall all git hooks managed by rusky.

```bash
npx @gunw.dan/rusky uninstall
```

## 🎯 Supported Git Hooks

rusky supports all standard Git hooks:

- `pre-commit` - Before commit
- `prepare-commit-msg` - Before commit message editor
- `commit-msg` - After commit message
- `post-commit` - After commit
- `pre-rebase` - Before rebase
- `post-checkout` - After checkout
- `post-merge` - After merge
- `pre-push` - Before push
- `pre-receive` - Before receive (server-side)
- `update` - Before update reference (server-side)
- `post-receive` - After receive (server-side)
- `post-update` - After update reference (server-side)
- `push-to-checkout` - Before push to checkout
- `pre-auto-gc` - Before auto gc
- `post-rewrite` - After rewrite

## 🔧 Configuration

rusky stores its configuration in `.rusky/config.json`:

```json
{
  "hooks": {
    "pre-commit": "npm test",
    "pre-push": "npm run lint"
  },
  "version": "0.1.0"
}
```

## 🆚 Why choose rusky?

**Simple, reliable, and dependency-free Git hooks management**

| Feature | rusky | husky |
|---------|-------|-------|
| **Setup** | 🚀 One command | 📚 Multiple steps |
| **Dependencies** | 🚫 Zero runtime | 📦 Multiple Node.js deps |
| **Learning Curve** | 📖 Same as husky | 📖 Same as rusky |
| **Maintenance** | 🔧 Less to manage | 🔧 More dependencies |
| **Language** | 🦀 Rust | 📜 Node.js |
| **Type Safety** | 🛡️ Compile-time | ⚠️ Runtime |

> **Perfect for**: Teams who want the same husky experience with fewer dependencies and simpler maintenance.

## 🛠️ Development

### Prerequisites
- Rust 1.70+
- Node.js 14+

### Building from source

```bash
# Clone the repository
git clone https://github.com/geonhwiii/rusky.git
cd rusky

# Build the project
cargo build --release

# Run tests
cargo test

# Install locally
npm install
```

### Running tests

```bash
# Run all tests
cargo test

# Run specific test modules
cargo test config::tests        # Config module tests
cargo test cli::tests          # CLI module tests  
cargo test hooks::tests        # Hooks module tests
cargo test git::tests          # Git module tests

# Run integration tests
cargo test --test integration_tests

# Run unit tests only (no integration tests)
cargo test --lib

# Run tests with output
cargo test -- --nocapture
```

### Test Coverage

rusky includes comprehensive test coverage:

- **Unit Tests**: Test individual functions and modules
  - `config.rs`: Configuration management tests
  - `cli.rs`: CLI command logic tests
  - `hooks.rs`: Hook file management tests
  - `git.rs`: Git integration tests

- **Integration Tests**: End-to-end testing of CLI commands
  - Full workflow testing (init → add → commit → remove)
  - Error handling and edge cases
  - Cross-platform compatibility

- **Test Features**:
  - Isolated test environments using temporary directories
  - Git repository simulation for testing
  - Comprehensive error scenario coverage
  - Performance and functionality validation

## 📄 License

MIT © [geonhwiii](https://github.com/geonhwiii)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 🔗 Related Projects

- [husky](https://github.com/typicode/husky) - The original Git hooks manager
- [pre-commit](https://pre-commit.com/) - A framework for managing pre-commit hooks
- [commitlint](https://commitlint.js.org/) - Lint commit messages

---

Made with ❤️ and 🦀 by [geonhwiii](https://github.com/geonhwiii) 