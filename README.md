<div align="center">

# Git Automater

**An interactive Rust CLI that removes the friction from everyday Git.**

[![Crates.io](https://img.shields.io/crates/v/git-automater.svg)](https://crates.io/crates/git-automater)
[![Downloads](https://img.shields.io/crates/d/git-automater.svg)](https://crates.io/crates/git-automater)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

*10,000+ downloads on crates.io*

</div>

![demo](https://us-east-1.tixte.net/uploads/shyamsundhar.tixte.co/2023-11-12_18-16-40.gif)

---

## Why

Git is powerful but command-heavy. Writing a well-formed Conventional Commit, adding the right
license, generating a `.gitignore`, or picking the correct remote and branch to push to all mean
remembering exact syntax or looking things up.

Git Automater wraps those flows in a single fuzzy-searchable menu. You pick what you want to do,
it prompts for what it needs, and it runs the right Git command for you — no syntax to memorise.

## Features

| Feature | What it does |
|---|---|
| **Initialize repository** | Runs `git init` in the current directory |
| **Add files** | Stage a specific path or everything (`.`) |
| **Conventional Commits** | Guided commit builder — pick a type (`feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`), each mapped to an emoji, then add an optional scope, body, and footer |
| **Push** | Fuzzy-select the branch *and* remote from your actual repo, then pushes with `-u` |
| **License generator** | Fetches the live license list from the GitHub API, then writes the chosen license with your name and the current year filled in |
| **.gitignore generator** | Generates a `.gitignore` for your project |
| **Branch manager** | Create, switch, and manage branches interactively |
| **Remote manager** | Add, view, and manage remotes interactively |
| **Clear cache** | `git rm -r --cached .` for when `.gitignore` changes need to take effect |

Everything runs through a fuzzy-select prompt, so you can type a few characters instead of
scrolling.

## Install

**From crates.io (recommended)**

```bash
cargo install git-automater
```

**From source**

```bash
git clone https://github.com/ShyamSundhar1411/git-automater.git
cd git-automater
cargo install --path .
```

Requires [Rust and Cargo](https://www.rust-lang.org/tools/install).

## Usage

```bash
git-automater
```

That's it — you'll get an interactive menu. Use arrow keys or start typing to filter, then press
Enter.

## Built with

- [Rust](https://www.rust-lang.org/) (2021 edition)
- [dialoguer](https://crates.io/crates/dialoguer) — fuzzy-select prompts and theming
- [reqwest](https://crates.io/crates/reqwest) — GitHub API calls for license fetching
- [serde](https://crates.io/crates/serde) — JSON deserialisation
- [chrono](https://crates.io/crates/chrono) — year resolution for licenses
- [emojis](https://crates.io/crates/emojis) — commit-type emoji mapping

## Contributing

Issues and pull requests are welcome.

1. Fork the repo
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes
4. Push and open a Pull Request

Please open one PR per change, and check spelling and formatting before submitting.

## License

MIT — see [LICENSE](LICENSE).
