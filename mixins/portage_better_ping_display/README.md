# portage-better-ping-display 0.1.1

Unofficial community Rust port of Better Ping Display — Fabric Edition,
originally created by Vlad Marica, under the MIT license.

Hold Tab in-game to see colored numeric latency after player names.
Enable the mod in Options > Mods. Connection bars remain visible.
The mod displays the value sent by the server and sends no extra ping requests.

This native package must be built into Horizon. It targets Minecraft 26.3 and
the Horizon revision in horizon-mixin.toml. Remove the initial better_ping_display
package before installing this one to avoid duplicate text.

The port does not include the original YACL screen, JSON settings or exact text
placement. The project archive includes the complete user guide, images and
source provenance.

Original author: Vlad Marica — Copyright (c) 2020 Vlad Marica.
https://github.com/vladmarica/better-ping-display-fabric
Complete license: LICENSE.

Name formatting adapted from Horizon — Copyright (c) 2026 Ludovic Marie and the
Horizon contributors. Complete license: LICENSE-Horizon. See NOTICE.md.
