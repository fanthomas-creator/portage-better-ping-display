# Security policy

## Supported releases

The latest reviewed 0.1.x release is the current maintenance line. Only the exact Horizon revisions listed in a release are supported. Nightly compatibility is not implied.

This is a native Rust mod compiled into the game. It is not sandboxed. Its intended behavior is limited to reading player-list latency and changing its display.

## Reporting a vulnerability

Use the repository's **Security → Report a vulnerability** feature when it is enabled. If private reporting is unavailable, open an issue containing only a short request for a private contact route, without exploit details, credentials or personal data. For account compromise, use GitHub's account-security support.

Do not post private keys, tokens, player data or a working exploit in a public issue. A suspected signing-key leak requires an immediate pause in releases and coordination with the Horizon registry maintainers before replacing the published public key.

## Release authenticity

Official project packages must have a valid minisign signature under the public key in `signing/portage-better-ping-display.pub`. A checksum detects changed bytes but does not replace signature verification. See [release instructions](docs/RELEASING.md).
