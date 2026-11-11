# Contributing

Thanks for helping maintain this unofficial port. Open an issue before proposing a new feature or a change to how the game is modified.

## Scope

Keep the mod focused on displaying server-provided player-list latency. Do not add telemetry, background services, advertisements, unrelated gameplay changes or additional network measurements.

Use a branch or fork and submit a pull request. Keep changes small and explain the user-visible behavior, its compatibility impact and how it was checked. The maintainer decides which changes are accepted. Opening a pull request does not grant write access.

## Required checks

```sh
cargo fmt --check
cargo test --locked
python scripts/check_project.py
python scripts/package.py
```

The lightweight CI checks formatting, the independent color logic, project structure and reproducible packaging. It does **not** prove that the entire game builds or runs on a new Horizon version. A renderer or hook change also needs an integration build on the declared revision and an in-game check. Record that evidence and never claim untested Nightly support.

## Native code and review

- Preserve original-author copyright and license notices.
- Use only source code whose license permits the adaptation and document its origin in `SOURCES.json` and the notices.
- Do not add dependencies, shell execution, arbitrary file access or network access without a specific reason and maintainer review.
- Do not commit credentials, private signing keys, compiled game executables, Minecraft resources, player saves or compiler caches.
- Do not alter compatibility pins, security rules, workflow permissions or the public signing key as an incidental part of another change.
- Report tests that were actually run, including failures and limitations.
- Label AI-assisted contributions and identify which tool assisted with them. Contributors remain responsible for reviewing their patches.

## Releases

Only the maintainer publishes releases. Every published version is immutable. Build from a reviewed commit, validate the package, then sign its exact bytes with the existing project key. The private key stays outside GitHub and CI. See [release instructions](docs/RELEASING.md).

Contributions are submitted under the project's MIT license. Follow the [code of conduct](CODE_OF_CONDUCT.md).
