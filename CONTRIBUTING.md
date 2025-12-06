# Contributing to cardano-installer

Thanks for your interest in contributing! This project is intended to be collaborative and beginner-friendly — the guidance below will help you get started and contribute in a safe, consistent way.

---

## Table of contents
- Getting started (setup and build)
- Running the app for development
- Testing
- Adding Functionality and Fixes
- Style & linters
- Creating Pull Requests
- Issue & Feature Requests
- Reviewing & Guidance for maintainers

---

## Getting started (Setup and build)

1. Clone the repository and switch to main branch:

```bash
git clone <repo-url>
cd cardano-installer
git checkout main
```

2. Make sure you have the Rust toolchain installed (recommended: rustup to manage toolchain).

3. Build and verify that the project compiles:

```bash
cargo build
```

4. Development dependencies used in this project (if you need to install them manually):
- `cargo fmt`/`rustfmt` (for formatting)
- `cargo clippy` (linting)


## Running the app for development

Run the app locally with the `--verbose` flag to print extra information while developing:

```bash
cargo run -- --verbose
```

When developing, you can also run specific utilities (for example, module-level functions or tests) directly using `cargo test` or invoking the sample code in `main.rs`.


## Testing

- Run the unit test suite:

```bash
cargo test
```

- If you add network download code, add unit tests that mock HTTP responses instead of hitting the internet. Recommended crates to use:
  - `wiremock` for Rust-based HTTP stubbing
  - `reqwest::blocking` can be swapped for a mocked client in tests

- For integration tests, prefer a `tests/` directory. Avoid relying on Docker or remote services in CI unless properly mocked / stubbed.


## Adding Functionality and Fixes

When adding a new feature or fix:
1. Create a new branch from `main`:

```bash
git checkout -b feat/<short-description>
```

2. Implement the feature in a focused commit(s). Small commits that explain a single change are better.

3. Add or update unit tests and documentation where appropriate.

4. Run formatter and linter:

```bash
cargo fmt
cargo clippy -- -D warnings
```

5. Ensure your code compiles and tests pass:

```bash
cargo build --all
cargo test --all
```

6. Push your branch and create a PR describing the change.


### Helpful places to contribute
- `src/configs/mithril.rs` — improvements for Mithril config parsing and discovery.
- `src/configs/download_file.rs` or `src/utils/download_file.rs` — improve downloads to handle Content-Disposition headers, extract tarballs, and fix cross-platform issues (e.g., setting executable permissions on Linux/macOS).
- `src/main.rs` — finish the user flows for Docker installation and snapshot flows.
- Tests and CI — add mocked tests, unit coverage, and automated checks.


## Style & linters

This project uses standard Rust tooling. Please follow these guidelines:
- Run `rustfmt` before submitting code.
- Run `cargo clippy -- -D warnings` and fix warnings (warnings are treated as errors in CI).
- Use idiomatic Rust patterns and `anyhow` for error handling where appropriate.


## Creating Pull Requests

When opening a PR:
- Give the PR a succinct title.
- Describe what the PR changes and why (add context and how to test it).
- Include a checklist example in your PR description. Example:

```
- [ ] I have added/updated tests
- [ ] I ran `cargo fmt` and `cargo clippy`
- [ ] The code compiles with `cargo build`
- [ ] Documentation updated (README/other relevant docs)
```

- If the change is large, consider opening an RFC/discussion issue first.


## Issue & Feature Requests

- Use GitHub Issues to propose features or report bugs.
- When opening an issue, include: steps to reproduce (if a bug), expected vs. actual behavior, and `cargo build` output if relevant.


## Reviewing & Maintainer Guidance

- Use labels to an issue (e.g., `good first issue`, `help wanted`) when triaging.
- Ask contributors to re-run `cargo fmt` / fix clippy warnings before merge.
- Keep PRs small and target a single purpose if possible.
- Be explicit about release notes when merging breaking changes.

---

## Branching & Releases

- Use `main` for stable code that can be released at any time.
- Use feature branches `feat/<what>` for new features.
- Merge into `main` once tests are passing and reviewers approve.

---

## Developer Tips

- Use `cargo watch` to iterate quickly on changes (`cargo watch -x 'run'`).
- If you are adding or modifying network calls, prefer `wiremock` to stub services in tests rather than hitting the network.
- Add a short note in your PR about the testing strategy for network or IO-heavy changes.

---

Thanks for helping to improve `cardano-installer`! If you have any questions or need help picking a task, open an issue and tag the maintainers.
