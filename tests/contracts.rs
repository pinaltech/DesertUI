use desert_ui::{Accent, Appearance, Colors, LayoutPolicy};

#[test]
fn pane_transitions_preserve_navigation_at_every_boundary() {
    for policy in [LayoutPolicy::default(), LayoutPolicy::IRONWOOD] {
        for width in [
            360., 479., 480., 600., 779., 780., 899., 900., 999., 1000., 1179., 1180., 1440.,
        ] {
            for selected in [false, true] {
                for collapsed in [false, true] {
                    let layout = policy.resolve(width, selected, collapsed);
                    assert!(layout.show_list || layout.show_detail);
                    assert_eq!(
                        layout.show_list && layout.show_detail,
                        width >= policy.single_below
                    );
                    if layout.single {
                        assert_eq!(layout.show_detail, selected);
                        assert_eq!(layout.list_width, None);
                    }
                    assert_eq!(
                        layout.sidebar_width,
                        if collapsed || width < policy.rail_below {
                            60.
                        } else {
                            228.
                        }
                    );
                    if width < 480. && selected {
                        assert!(!layout.show_sidebar);
                    }
                    if width >= 480. {
                        assert!(layout.show_sidebar);
                    }
                }
            }
        }
    }
    assert!(
        LayoutPolicy::default()
            .resolve(360., false, false)
            .show_sidebar
    );
    assert!(
        !LayoutPolicy::IRONWOOD
            .resolve(360., false, false)
            .show_sidebar
    );
}
fn luminance(rgb: u32) -> f64 {
    let channel = |shift: u32| {
        let s = f64::from((rgb >> shift) & 255u32) / 255.;
        if s <= 0.04045 {
            s / 12.92
        } else {
            ((s + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * channel(16) + 0.7152 * channel(8) + 0.0722 * channel(0)
}
#[test]
fn every_palette_keeps_readable_accent_text_and_stable_preference_keys() {
    for accent in Accent::ALL {
        assert_eq!(Accent::from_key(accent.key()), accent);
        for dark in [false, true] {
            let c = Colors::new(dark, accent);
            for background in [c.accent, c.accent_hover] {
                let a = luminance(background);
                let b = luminance(c.on_accent);
                let contrast = (a.max(b) + 0.05) / (a.min(b) + 0.05);
                assert!(
                    contrast >= 4.5,
                    "{} dark={dark}: {contrast}",
                    accent.label()
                );
            }
        }
    }
    assert_eq!(Accent::from_key("legacy-unknown"), Accent::Green);
    assert_eq!(Appearance::System.key(), "system");
    assert_eq!(Appearance::Light.key(), "light");
    assert_eq!(Appearance::Dark.key(), "dark");
}
