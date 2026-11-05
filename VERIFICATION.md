# Verification

## Library interaction components — October 31, 2026

Added shared menu surfaces and toolbar separators, application-independent
image fit/zoom/pan geometry, and macOS file-drag/magnification helpers. Existing
component signatures and behavior remain unchanged.

Formatting, locked offline tests (**4 pass**), and strict all-target Clippy pass.
Notes has **18 passing tests** and Photos **13**; both native app bundles build.
Picacho and the Ironwood workspace pass locked offline compilation with the
shared additions. Their full suites were not rerun for this update.

Native app review used isolated fictional libraries. Native photo drag-session
creation and an in-app album drop were verified. A complete Finder copy,
physical trackpad pinch, and Linux/Windows native acceptance remain unverified.
The app-specific verification files give coverage details. No browser was used;
desktop control stopped when the user requested it.


## Library application addition — October 31, 2026

Added the `library` module for GPUI Notes and GPUI Photos: composable SVG assets,
toolbar tools, sidebar rows, source/onboarding cards, empty states, badges, and
headings. Existing component behavior is unchanged. Formatting, the 3 existing
tests, and strict all-target Clippy pass. Notes adds 16 passing tests; Photos
adds 12. Both have native debug app bundles and were visually reviewed using
isolated fictional libraries. Their READMEs and VERIFICATION.md files record
features, storage semantics, supported formats, tested sizes, and limitations.
Picacho and the Ironwood workspace pass `cargo check --offline`; only their
lockfiles changed to record DesertUI's added assets dependency. Their complete
test suites were not rerun for this addition. The migration results below are
the earlier baseline, retained for context.

October 30–31, 2026, Apple Silicon macOS, Rust 1.96.0. UI reference audit was
performed before migration: [UI audit](docs/UI-AUDIT.md).

| Repository | Checks | Result |
| --- | --- | --- |
| DesertUI | Formatting; locked tests; strict all-target Clippy, including gallery | 3 tests pass |
| Picacho | Formatting; locked default and all-feature suites; strict all-target/all-feature Clippy; native debug build with sample data | 57 pass, 2 opt-in tests ignored in each suite |
| Ironwood | Formatting; locked workspace suite; strict workspace/all-target/all-feature Clippy; native app and host debug builds | 47 pass, 2 opt-in tests ignored |

DesertUI tests cover both layout policies at every breakpoint and neighboring
widths, stable appearance/accent keys, 4.5:1 or better contrast for every accent
foreground against normal/hover backgrounds, actual GPUI list/detail/Back clicks,
native-toolbar-sized settings controls, and settings geometry at 1440×1000,
1180×700, 960×600, 899×600, 700×480, 640×480, 480×420 and 360×320.

Existing application tests remain active; only fixture socket setup changed. Picacho's UI tests cover
onboarding, sidebar accounts, bulk actions, compose, HTML controls, encrypted-reader
unlock feedback, per-account errors and settings scrolling. Ironwood's 17 GPUI
tests cover create/edit/save, search, clipboard/reveal, deletion/lock confirmation,
wrong passwords, toolbar controls, keyboard shortcuts, settings preferences, focus
containment/restoration, generator, provider forms, accounts and responsive bounds.
Its remaining workspace tests cover the host contract, storage, imports, crypto
and local provider synchronization. No live service credentials were used.

Native visual acceptance used separately named `.app` review bundles and isolated
profiles under `/private/tmp/desert-ui-native-review`. Copied native executables
were checked against the rebuilt originals by SHA-256 before launch. Inspected
onboarding/unlock, three-pane navigation, message reading, light/dark settings,
account dropdown anchoring and native toolbar Settings activation. Native pointer
checks used 100–200ms press durations so GPUI processes down/up in separate frames.
Responsive minimum sizes were verified by the real GPUI layout tests, not by
claiming the desktop screenshots were captured at those sizes.

Before migration, Ironwood's workspace passed. Picacho's parallel and serial
baselines both failed eight existing TLS-fixture tests (49 passed, 2 ignored).
The accepted-socket fix described in [migration notes](docs/MIGRATION.md) makes
all eight pass. The same inherited-socket correction also fixes an intermittent
HTML resource-server fixture failure. Production networking is unchanged.

Limits: Linux/Windows native acceptance, live discovery, Dovecot and opt-in live/
official-provider interoperability tests were not run. Release packaging and
store-installed browser extensions were not revalidated in this UI extraction.
Cargo reports existing dependency future-incompatibility notices for `block`
and (Picacho) `imap-proto`; strict Clippy and compilation still pass.

Repeat the automated validation with `./scripts/check.sh` from a sibling-checkout
layout. The library is a local path dependency; it has not been published or pushed.
