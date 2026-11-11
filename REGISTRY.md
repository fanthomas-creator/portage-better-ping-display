# Horizon registry submission

This file prepares the form's contents. It does not publish anything.

## Fields

| Field | Value |
| --- | --- |
| Type | Native mod |
| File | `dist/portage-better-ping-display-0.1.1.hnmod` |
| ID | `portage_better_ping_display` |
| Public name | `portage-better-ping-display` |
| Version | `0.1.1` |
| Minecraft | `26.3` |
| License | `MIT` |
| Original author | `Vlad Marica` |
| Category | `Utility` |
| Tags | `ping, latency, tab-list, client-side, quality-of-life` |
| Optional icon | `icon.png`: 400 Ã— 400 PNG |
| Port author or maintainer | `fanthomas-creator` |
| Port source | `https://github.com/fanthomas-creator/portage-better-ping-display` |
| Summary | Colored ping in milliseconds in Tab: an unofficial community Rust port of Better Ping Display for Horizon. |
| What changed? | Renamed the project and native ID; added English documentation, original-author credits, a user guide and images. Display behavior is unchanged. |

The source field must link to **this Rust port**, not only the original Java repository. Keep the original author's credits in the description and license notices.

## Description to copy and adapt

**portage-better-ping-display** is an unofficial community port of **Better Ping Display â€” Fabric Edition**, originally created by **Vlad Marica**.

Hold **Tab** to show latency in milliseconds after player names. Text blends from green through yellow to red as latency increases. Connection bars and name formatting are preserved. Toggle the mod in **Options â†’ Mods**.

The displayed value comes from the server; the mod sends no extra ping requests. Updates depend on the server, generally every 30 seconds on vanilla at its normal tick rate.

This Rust adaptation does not include the original YACL screen or JSON settings. It uses `<value>ms` and leaves connection bars visible. As a native mod, it must be built into Horizon through your integration.

Target: Minecraft 26.3, Horizon `fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca`. Other Nightly revisions require compatibility checks.

**Credits:** Vlad Marica, original mod and color logic, MIT license. Adapted Horizon rendering portions: Ludovic Marie and the Horizon contributors, MIT license. Complete notices and source provenance are included in the project and native package.

Original mod: https://github.com/vladmarica/better-ping-display-fabric

## Images

- `docs/images/ping-in-game.png`: real screenshot supplied by the user, from the initial 0.1.0 release. Display behavior is preserved in 0.1.1.
- `docs/images/ping-colors.png`: color reference diagram; an editable SVG is also included.

The small cropped in-game image is for the README: it is below the registry gallery's minimum dimensions and must not be uploaded as a screenshot. The 960 Ã— 300 color diagram fits the gallery's image dimensions, but must be identified as an illustration, not a gameplay screenshot. Gameplay screenshots are optional; take a full-sized capture if you want to add one.

The form accepts up to eight PNG/JPEG gallery images, at most 1 MiB each, from 320 Ã— 180 to 3840 Ã— 2160 pixels. The icon is a separate square PNG field, 64â€“1024 pixels and at most 256 KiB; the supplied 400 Ã— 400 `icon.png` fits those limits.

Relative image links work in the repository README. For the registry's Markdown form, use public image URLs from the port's repository once it has been published. Use `icon.png` for the optional icon field; its editable source is `docs/images/icon.psd`.

## Before submitting

1. Publish these sources in a public repository and enter its URL and the port maintainer's name.
2. Check the current Nightly before claiming support for it; the included validation covers only the stated revisions.
3. Validate the `.hnmod` with `cargo horizon-mod check`; its hash is recorded in `VALIDATION.json`.
4. Sign in to the registry form with your GitHub account and upload the `.hnmod`, not the documentation ZIP.

The signed package's `.minisig` is in `dist` and its public key is in `signing/portage-better-ping-display.pub`. Copy the last non-comment line of the `.pub` file into **Public key**. Upload the `.minisig` to **Signature**, together with the exact matching `.hnmod` package. Never upload the private key.

Submission opens a pull request; publication follows maintainer review and merging.

Reference: https://playhorizon.dev/docs/publish-a-mod â€” accessed November 11, 2026.

## Signature fields for this release

Public key:

```text
RWRau4lakVV3M+l+S07QhsxXTFj25ohW+TNQWv609tmh1D7PNLDrxLHd
```

Signature file: `dist/portage-better-ping-display-0.1.1.hnmod.minisig`. Upload the matching `.hnmod` file from the same directory.
