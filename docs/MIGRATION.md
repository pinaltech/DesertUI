# Application migration

Both apps depend on the same `desert-ui` 0.1.0 path crate. There is no copied
component source or second toolkit instance in either application.

| Shared concern | Picacho | Ironwood |
| --- | --- | --- |
| Initialization | `desert_ui::init` | `desert_ui::init` |
| Theme and accents | Shared types; existing string preference keys | Shared types; existing numeric preferences translated to accent enum |
| Buttons and SVGs | Shared surfaces/icons; mail actions and icon paths stay local | Shared surfaces/icons; keyboard activation, focus, actions and icon paths stay local |
| Topbar/sidebar | Shared surfaces, metrics and toolbar sizing | Shared surfaces and metrics |
| Responsive panes | Default Picacho policy | Explicit `LayoutPolicy::IRONWOOD`; existing 330px list and mobile bottom navigation |
| Settings | Shared navigation/shell/page; existing scroll handle retained | Shared navigation/shell/page; General/Security content retained |
| Dialogs | Shared bounded panels/backdrop; compose, account and reader content local | Shared bounded panels/backdrop; focus trap, restoration and security actions local |
| Dropdowns | Shared popup surface; anchoring, entries and dismissal local | Shared popup surface; account ownership, anchoring and dismissal local |
| Native titlebar | Shared AppKit forwarding installed once and synchronized during render | Existing forwarding moved into the crate |

The migration deliberately leaves domain-specific rendering in each app: mail
message rows and HTML rendering, vault details, providers, compose/editor flows,
security/session state, storage formats, synchronization, assets, keyboard maps
and animations. Further extraction should be driven by an actual shared use case.

The library's `toolbar_button` consumes left mouse-down so native titlebar
handling does not start dragging a window on a control press. Real macOS
window testing is required alongside GPUI layout tests: GPUI's TestWindow has no
AppKit handle. Render-time native calls are guarded with `#[cfg(not(test))]`.
Other platforms get no-op native-titlebar functions.

Picacho's pre-existing macOS TLS-fixture failures were traced to accepted sockets
inheriting the fixture listener's nonblocking flag. Setting those accepted sockets
to blocking mode makes the test harness match its blocking TLS protocol code and
explicit read/write deadlines. The HTML resource fixture receives the same accepted-socket correction after
an intermittent macOS WouldBlock failure. No production transport or TLS
verification changed.

Preserved app baselines: Picacho `949974f` (already clean, so an explicit empty
checkpoint); Ironwood `2ed0854` (all existing work committed before migration).
