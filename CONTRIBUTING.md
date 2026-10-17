# Contributing

Thanks for contributing to ReproCrate. The project is open source under [EPL-2.0](LICENSE).

1. Discuss substantial changes in an issue before starting.
2. Use stable Rust 1.82+ and run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings` and `cargo test --workspace`.
3. Add tests for behavior and hostile input when relevant; do not include private logs or secrets in fixtures.
4. Send a focused pull request explaining validation and privacy impact.
5. Preserve attribution and notices; contributions are submitted under the project's EPL-2.0 terms, and upstream third-party code remains subject to its own license.

Commit messages may use conventional prefixes such as `feat(core):`, `test(cli):`, `fix(capture):`, `docs:`. Commit email for project-owned work is `sanskarin@outlook.in`. Do not rewrite others' authorship.
