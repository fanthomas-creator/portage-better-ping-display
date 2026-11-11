# Credits and provenance

## Better Ping Display — Fabric Edition

- Original author: **Vlad Marica**.
- Preserved copyright: **Copyright (c) 2020 Vlad Marica**.
- License: **MIT**; the full notice is included in `LICENSE` and the native bundle.
- Repository: https://github.com/vladmarica/better-ping-display-fabric
- Source revision used: `96ddda664ff4c4eb2e119c22d3ed6e93b1012294`.

This port follows the original numeric player-list latency display and adapts the palette and interpolation from `PingColors.java` and `ColorUtil.java`. `CustomPlayerListHud.java` served as a reference for display behavior. Java code is not executed in Horizon; integration is rewritten in Rust.

The name **portage-better-ping-display** identifies a community adaptation. It does not claim to be a release published or endorsed by the original author.

## Horizon

- Copyright: **Copyright (c) 2026 Ludovic Marie and the Horizon contributors**.
- License: **MIT**; the full notice is included in `LICENSE-Horizon` and the native bundle.
- Repository: https://github.com/horizon-rust/horizon
- Target revision: `fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca`.

The name-formatting code in `name.rs` adapts `name_for_display` from `crates/horizon-client-render/src/tab_list.rs`.

## Port and documentation

Port maintainer: **[fanthomas-creator](https://github.com/fanthomas-creator)**. Rust adaptation and documentation prepared with assistance from **Codex** for this community project. Original-author credit and copyright remain with Vlad Marica and the respective Horizon contributors.

`images/ping-in-game.png` was supplied by the user in this conversation and shows the initial 0.1.0 release working in-game. The color diagram is a documentation illustration, not a screenshot.

The project icon was supplied by the port maintainer as `icon.psd`. The supplied artwork was exported to `icon.png` without redesigning it. Its editable Photoshop source is included in `docs/images/icon.psd`.

The illustration uses the original author's color values to explain the port. This project does not distribute Minecraft game files or resources.
