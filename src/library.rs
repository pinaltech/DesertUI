//! Library application surfaces. Hosts own navigation, source drivers and data.
use crate::{Colors, button, icon};
use gpui::{prelude::*, *};
use gpui_component::tooltip::Tooltip;
use std::borrow::Cow;

/// Shared icon namespace, composed with GPUI Component's built-in assets.
pub struct LibraryAssets;
impl AssetSource for LibraryAssets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if let Some(name) = path.strip_prefix("library/") {
            let shape = match name.trim_end_matches(".svg") {
                "notes" => {
                    "<rect x='4' y='3' width='16' height='18' rx='3'/><path d='M4 8h16M8 12h8M8 16h6'/>"
                }
                "photos" => {
                    "<rect x='3' y='3' width='18' height='18' rx='4'/><circle cx='8' cy='8' r='1.5'/><path d='m3 17 6-6 4 4 3-3 5 5'/>"
                }
                "folder" => {
                    "<path d='M3 7V5a2 2 0 0 1 2-2h5l2 3h7a2 2 0 0 1 2 2v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2Z'/>"
                }
                "all" => {
                    "<rect x='3' y='3' width='7' height='7' rx='1.5'/><rect x='14' y='3' width='7' height='7' rx='1.5'/><rect x='3' y='14' width='7' height='7' rx='1.5'/><rect x='14' y='14' width='7' height='7' rx='1.5'/>"
                }
                "pin" => "<path d='m16 3 5 5-4 2-3 6-6-6 6-3ZM8 16l-5 5'/>",
                "heart" => {
                    "<path d='M20.8 4.6a5.5 5.5 0 0 0-7.8 0L12 5.7l-1.1-1.1a5.5 5.5 0 0 0-7.8 7.8L12 21l8.8-8.6a5.5 5.5 0 0 0 0-7.8Z'/>"
                }
                "trash" => "<path d='M3 6h18M9 6V3h6v3M5 6l1 15h12l1-15M10 10v7M14 10v7'/>",
                "plus" => "<path d='M12 4v16M4 12h16'/>",
                "edit" => {
                    "<path d='M12 4H5a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2h13a2 2 0 0 0 2-2v-7M16 3a2.1 2.1 0 0 1 3 3L10 15l-4 1 1-4Z'/>"
                }
                "search" => "<circle cx='10.5' cy='10.5' r='6.5'/><path d='m16 16 5 5'/>",
                "sidebar" => "<rect x='3' y='4' width='18' height='16' rx='3'/><path d='M9 4v16'/>",
                "back" => "<path d='m15 5-7 7 7 7'/>",
                "next" => "<path d='m9 5 7 7-7 7'/>",
                "down" => "<path d='m6 9 6 6 6-6'/>",
                "close" => "<path d='m6 6 12 12M6 18 18 6'/>",
                "settings" => {
                    "<path d='M4 7h16M4 17h16'/><circle cx='9' cy='7' r='3'/><circle cx='15' cy='17' r='3'/>"
                }
                "cloud" => {
                    "<path d='M7 18a5 5 0 0 1-1-9.9 6 6 0 0 1 11.5-1.4A5.5 5.5 0 0 1 18 18Z'/>"
                }
                "drive" => "<path d='M5 3h14l3 13v4H2v-4Z'/><path d='M2 16h20M16 18h2'/>",
                "check" => "<path d='m5 12 4 4L19 6'/>",
                "refresh" => {
                    "<path d='M20 7a9 9 0 0 0-15-2L2 8M2 3v5h5M4 17a9 9 0 0 0 15 2l3-3M22 21v-5h-5'/>"
                }
                "import" => "<path d='M12 3v12m-5-5 5 5 5-5M4 16v5h16v-5'/>",
                "export" => "<path d='M12 16V3m-5 5 5-5 5 5M4 16v5h16v-5'/>",
                "tag" => "<path d='M3 3h8l10 10-8 8L3 11Z'/><circle cx='7.5' cy='7.5' r='1'/>",
                "eye" => {
                    "<path d='M2 12s4-7 10-7 10 7 10 7-4 7-10 7S2 12 2 12Z'/><circle cx='12' cy='12' r='3'/>"
                }
                "code" => "<path d='m8 6-6 6 6 6M16 6l6 6-6 6M14 3l-4 18'/>",
                "info" => "<circle cx='12' cy='12' r='9'/><path d='M12 11v6M12 7h.01'/>",
                "link" => {
                    "<path d='m10 13 4-4M8 16l-1 1a4 4 0 0 1-6-6l5-5a4 4 0 0 1 6 0M16 8l1-1a4 4 0 0 1 6 6l-5 5a4 4 0 0 1-6 0'/>"
                }
                "restore" => "<path d='M3 4v6h6M3 10a9 9 0 1 1 1 8M12 7v5l3 2'/>",
                "minus" => "<path d='M5 12h14'/>",
                _ => return Ok(None),
            };
            return Ok(Some(Cow::Owned(format!("<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='1.6' stroke-linecap='round' stroke-linejoin='round'>{shape}</svg>").into_bytes())));
        }
        gpui_component_assets::Assets.load(path)
    }
    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        gpui_component_assets::Assets.list(path)
    }
}

pub fn glyph(name: &str, size: f32, color: u32) -> Div {
    icon(format!("library/{name}.svg"), size, color)
}

pub fn tool(id: &'static str, name: &str, label: &'static str, c: Colors) -> Stateful<Div> {
    button(id, c, false, true)
        .w(px(32.))
        .h(px(30.))
        .px_0()
        .bg(transparent_black())
        .child(glyph(name, 17., c.muted))
        .tooltip(move |w, cx| Tooltip::new(label).build(w, cx))
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
}

pub fn section(label: impl Into<SharedString>, c: Colors) -> Div {
    div()
        .px_3()
        .pt_5()
        .pb_2()
        .text_size(px(11.))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(rgb(c.muted))
        .child(label.into())
}

pub fn nav_item(
    id: impl Into<SharedString>,
    name: &str,
    label: impl Into<SharedString>,
    count: Option<usize>,
    selected: bool,
    c: Colors,
) -> Stateful<Div> {
    button(id, c, false, true)
        .w_full()
        .justify_start()
        .h(px(34.))
        .px_3()
        .gap_2()
        .bg(if selected {
            rgb(c.soft).into()
        } else {
            transparent_black()
        })
        .text_color(rgb(if selected { c.accent } else { c.text }))
        .child(glyph(name, 17., if selected { c.accent } else { c.muted }))
        .child(div().flex_1().min_w_0().truncate().child(label.into()))
        .when_some(count, |d, n| {
            d.child(
                div()
                    .text_size(px(11.))
                    .text_color(rgb(c.muted))
                    .child(n.to_string()),
            )
        })
}

pub fn badge(text: impl Into<SharedString>, c: Colors) -> Div {
    div()
        .flex_shrink_0()
        .px_2()
        .py_1()
        .rounded(px(6.))
        .bg(rgb(c.soft))
        .text_size(px(10.))
        .text_color(rgb(c.accent))
        .child(text.into())
}

pub fn empty(
    name: &str,
    title: impl Into<SharedString>,
    detail: impl Into<SharedString>,
    c: Colors,
) -> Div {
    let detail = detail.into();
    div()
        .size_full()
        .min_h_0()
        .p_5()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_3()
        .child(
            div()
                .size(px(64.))
                .rounded(px(19.))
                .bg(rgb(c.soft))
                .flex()
                .items_center()
                .justify_center()
                .child(glyph(name, 30., c.accent)),
        )
        .child(
            div()
                .text_size(px(20.))
                .font_weight(FontWeight::SEMIBOLD)
                .child(title.into()),
        )
        .when(!detail.is_empty(), |d| {
            d.child(
                div()
                    .max_w(px(340.))
                    .text_center()
                    .text_size(px(13.))
                    .line_height(relative(1.6))
                    .text_color(rgb(c.muted))
                    .child(detail),
            )
        })
}

/// A source choice remains visually distinct from connected sources.
pub fn source_card(
    id: &'static str,
    name: &str,
    title: &'static str,
    detail: &'static str,
    available: bool,
    c: Colors,
) -> Stateful<Div> {
    div()
        .id(id)
        .debug_selector(move || id.into())
        .w_full()
        .p_4()
        .flex()
        .items_center()
        .gap_3()
        .rounded(px(12.))
        .border_1()
        .border_color(rgb(c.border))
        .bg(rgb(c.raised))
        .when(available, |d| {
            d.cursor_pointer()
                .hover(move |s| s.border_color(rgb(c.accent)).bg(rgb(c.soft)))
        })
        .child(
            div()
                .size(px(38.))
                .rounded(px(10.))
                .bg(rgb(c.soft))
                .flex()
                .items_center()
                .justify_center()
                .child(glyph(name, 21., c.accent)),
        )
        .child(
            div()
                .flex_1()
                .min_w_0()
                .child(
                    div()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_size(px(13.))
                        .child(title),
                )
                .when(!detail.is_empty(), |d| {
                    d.child(
                        div()
                            .mt_1()
                            .text_size(px(12.))
                            .text_color(rgb(c.muted))
                            .child(detail),
                    )
                }),
        )
        .child(if available {
            glyph("next", 16., c.muted).into_any_element()
        } else {
            badge("Unavailable", c).into_any_element()
        })
}

pub fn welcome(
    name: &str,
    title: &'static str,
    subtitle: &'static str,
    c: Colors,
) -> Stateful<Div> {
    welcome_with(
        div()
            .mb_2()
            .size(px(68.))
            .rounded(px(20.))
            .bg(rgb(c.soft))
            .flex()
            .items_center()
            .justify_center()
            .child(glyph(name, 34., c.accent)),
        title,
        subtitle,
        c,
    )
}

/// `welcome`, headed by the app's full-color logo (an `img` asset path on the
/// shared 640×540 brand canvas) instead of a tinted glyph tile.
pub fn welcome_brand(
    logo: &'static str,
    title: &'static str,
    subtitle: &'static str,
    c: Colors,
) -> Stateful<Div> {
    welcome_with(
        div().mb_2().child(img(logo).w(px(96.)).h(px(81.))),
        title,
        subtitle,
        c,
    )
}

fn welcome_with(
    mark: impl IntoElement,
    title: &'static str,
    subtitle: &'static str,
    c: Colors,
) -> Stateful<Div> {
    div()
        .id("welcome")
        .debug_selector(|| "welcome".into())
        .w_full()
        .max_w(px(520.))
        .flex()
        .flex_col()
        .gap_3()
        .child(mark)
        .child(
            div()
                .text_size(px(32.))
                .font_weight(FontWeight::BOLD)
                .child(title),
        )
        .child(
            div()
                .mb_4()
                .text_size(px(14.))
                .line_height(relative(1.6))
                .text_color(rgb(c.muted))
                .child(subtitle),
        )
}

pub fn panel_heading(title: &'static str, subtitle: impl Into<SharedString>, c: Colors) -> Div {
    let subtitle = subtitle.into();
    div()
        .px_5()
        .py_4()
        .flex_shrink_0()
        .border_b_1()
        .border_color(rgb(c.border))
        .child(
            div()
                .text_size(px(20.))
                .font_weight(FontWeight::SEMIBOLD)
                .child(title),
        )
        .when(!subtitle.is_empty(), |d| {
            d.child(
                div()
                    .mt_1()
                    .text_size(px(12.))
                    .text_color(rgb(c.muted))
                    .child(subtitle),
            )
        })
}
