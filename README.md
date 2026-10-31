# DesertUI

A composable native application toolkit on GPUI 0.2.2 and GPUI Component 0.5.1,
extracted from Picacho and Ironwood. Crate name: `desert-ui`; Rust import:
`desert_ui`. Picacho is the reference for shared design decisions.

## Use from an application

Keep the three repositories side by side, then add:

```toml
[dependencies]
desert-ui = { version = "0.1.0", path = "../DesertUI" }
```

For a nested workspace member (Ironwood's `apps/native`), use `../../../DesertUI`.
The version requirement makes the intended API explicit; publication is not part
of this change. A path dependency requires the sibling source checkout to build.

```rust,ignore
use desert_ui::{Accent, Colors, LayoutPolicy};
use gpui::{prelude::*, *};

let colors = Colors::new(false, Accent::Green);
let layout = LayoutPolicy::default().resolve(
    window.viewport_size().width.into(), selected.is_some(), sidebar_collapsed,
);
let save = desert_ui::button("save", colors, true, false)
    .child("Save")
    .on_click(cx.listener(|this, _, window, cx| this.save(window, cx)));
```

Call `desert_ui::init(cx)` once before creating views. Initialize your GPUI
`Application` with an asset source (GPUI Component's assets plus app-specific
logos/icons), and wrap the main view in `gpui_component::Root` as usual.

## Components and ownership

- `Colors`, `Accent`, `Appearance`: one shared palette and stable preference keys.
- `LayoutPolicy`, `PaneLayout`, `split_panes`: responsive list/detail navigation.
- `button`, `icon_button`, `toolbar_button`, `icon`: composable controls and icons.
- `topbar`, `sidebar`: native application chrome with shared metrics.
- `modal_panel`, `modal_backdrop`: bounded, styled dialog surfaces.
- `settings_nav`, `settings_shell`, `settings_page`: responsive scrolling settings.
- `popup_surface`: shared dropdown presentation.
- `system_dark`, `shortcut_text`: platform appearance and shortcut presentation.
- `native_titlebar::install`, `native_titlebar::sync_layout`: macOS input forwarding
  for transparent 38px titlebars; no-ops on other platforms. Install once when
  creating each native window and sync during rendering to follow resizing.
  Guard the render-time call with `#[cfg(not(test))]` in headless GPUI tests;
  GPUI 0.2.2's test windows intentionally panic when asked for native handles.

Components return ordinary GPUI elements, with no global app state. Hosts retain
listeners, keyboard activation, tooltips, focus groups, dismissal policy, menu
anchoring, animation IDs, assets and app content. In particular, a modal surface
does **not** automatically trap focus or dismiss on backdrop clicks; callers
attach those policies. This preserves each application's tested behavior.

The default layout uses Picacho's breakpoints. `LayoutPolicy::IRONWOOD` preserves
Ironwood's existing pane and mobile-navigation behavior during migration.
Shared colors follow Picacho, including its `0xf8f9fb` light raised surface.
Vault encryption, protocols, message rendering, and secrets never enter this crate.

## Develop and verify

```sh
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo run --locked --example gallery
```

On macOS GPUI's supported runtime Metal shader compilation is enabled, so a
separate Xcode Metal Toolchain download is unnecessary. Linux/Windows use GPUI's
normal platform backend and prerequisites. Native validation here is on macOS;
other platforms require their own native acceptance run.

[UI audit](docs/UI-AUDIT.md) records both apps before migration.
[Verification](VERIFICATION.md) records actual checks and remaining limits.


Run `./scripts/check.sh` to repeat the automated checks across all three sibling
repositories. `DESERTUI_EMAIL_DIR` and `DESERTUI_VAULT_DIR` override their locations.
See [migration notes](docs/MIGRATION.md) for application integration boundaries.
