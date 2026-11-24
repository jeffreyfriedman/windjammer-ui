# Contributing to Windjammer UI

Thank you for helping make Windjammer UI better. The library pairs `.wj` component sources with a Rust reactive core (signals, keyed DOM patcher) that drives the browser loop.

## Ways to contribute

- Report bugs with a minimal reproduction.
- Suggest features and describe the problem they solve.
- Improve documentation and examples.
- Submit fixes, features and tests.
- Help others in issues and discussions.

## Before you start

1. Search the [issue tracker](https://github.com/jeffreyfriedman/windjammer-ui/issues) for related reports.
2. For anything larger than a small fix, open an issue first so we can agree on the approach.
3. Read the README and `AGENTS.md` for the project's conventions.

## Reporting bugs

Open an issue with a clear title, what you expected, what happened, a minimal reproduction, your `wj --version` output and operating system, and the complete error output.

## Development setup

```bash
git clone https://github.com/YOUR_USERNAME/windjammer-ui.git
cd windjammer-ui
git checkout -b feature/your-change
```

You need a recent stable Rust toolchain (see `rust-version` in `Cargo.toml`) and the `wj` compiler. Component sources live in `src/components_wj/`; the Rust reactive core is in `src/`.

## Testing

- Run the test suite: `cargo test`, or one target with `cargo test --test <name>`.
- Format and lint: `cargo fmt --all` and `cargo clippy -- -D warnings`.
- Keep component builders and their examples in step; `tests/README.md` describes the layout.

Every behavior change comes with a test that fails without the change.

## Public hygiene gate

This repository is public. A repo-local gate fails when a tracked file name or file content matches a forbidden-term pattern (private project names, personal paths). It is `tests/public_hygiene_test.rs` (`cargo test --test public_hygiene_test`), which reads the committed pattern list in `docs/public-hygiene-patterns.txt` and includes a self-test that proves it can fail.

## Pull requests

1. Rebase on the latest default branch.
2. Keep commits small and focused; use conventional prefixes (`feat`, `fix`, `docs`, `refactor`, `test`, `chore`).
3. Run the checks above and update documentation for user-facing changes.
4. Open the pull request, describe what changed and why, and link the related issue.
5. Respond to review comments; questions are always welcome.

## Code of Conduct and security

Participation is covered by our [Code of Conduct](CODE_OF_CONDUCT.md). Report vulnerabilities privately as described in [SECURITY.md](SECURITY.md).

## License

Contributions are licensed under the same terms as the project: MIT or Apache-2.0, at your option (see `LICENSE-MIT` and `LICENSE-APACHE`).
