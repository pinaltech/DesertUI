# UI audit before extraction

Inspected October 30, 2026, before editing either app. Sources are the sibling
Picacho (`GPUI Email Client`) and Ironwood (`GPUI Password Manager`) repositories.
Reviewed their actual GPUI render trees, layout and interaction tests, verification
records, and running native windows. The installed applications were used only
for read-only visual reference; automated tests use disposable profiles.

| Area | Picacho reference | Ironwood before migration |
| --- | --- | --- |
| Chrome | 38px topbar, 28px toolbar controls, macOS traffic-light inset 80px | Same geometry; separate native titlebar input forwarding |
| Three panes | Sidebar 228px, list 380px, remaining reader | Sidebar 228px, list 340px, remaining detail |
| Intermediate window | Below 1180px: 60px rail and 320px list | Below 1000px: 60px rail |
| Single content pane | Below 900px: list or reader with Back navigation | Below 780px: list or detail with Back navigation |
| Mobile | Below 480px reader hides sidebar; list retains rail | Below 480px bottom navigation replaces sidebar |
| Buttons | 36px high, radius 9px, icon-only width 34px; primary and surface styles | Same, plus focus/Enter/Space handling and action-dependent hover |
| Theme | Light/dark/system, seven accents, contrasting accent foregrounds | Duplicated palettes; light raised surface differs (white) |
| Settings | 820×600 bounded by viewport minus 32px; 210px navigation | Same dimensions and structure |
| Narrow settings | Below 700px, 56px horizontal scrolling navigation, padded scrolling page | Same |
| Dialogs | 18px radius, viewport bounds, overlay, scrollable content, entrance animation | Same; explicit keyboard focus containment and restoration |
| Menus | Anchored, window-snapped dropdowns, separators/checks/danger actions | Anchored account switcher, viewport snapping, scrolling, backdrop dismissal |
| Content | Search, virtualized mail list, reader, compose, onboarding | Search, item list, detail/editor, generator, onboarding |

Picacho wins shared design conflicts, including the light raised-surface color.
The existing application-specific responsive policies remain explicit parameters
so migration does not silently change which panes or navigation are reachable.
Application actions, keyboard maps, subscriptions, focus containment, assets,
animation state, mail protocols, vault secrets and persistence remain in the apps.

Baseline checks: Ironwood's complete workspace suite passes. Picacho's first
parallel run passes 49 tests (including all UI tests), fails eight local TLS
reliability tests, and ignores two opt-in tests. Serial rerun results are recorded
in VERIFICATION.md. No external account or live credentials are needed for this
migration's regression checks.
