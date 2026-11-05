use crate::{Colors, metrics};
use gpui::{prelude::*, *};

/// A button surface; add application-owned icon, text, tooltip and listeners.
/// Returning the concrete element preserves GPUI's fluent composition API.
pub fn button(
    id: impl Into<SharedString>,
    c: Colors,
    primary: bool,
    selected_hover: bool,
) -> Stateful<Div> {
    let id = id.into();
    let selector = id.to_string();
    div()
        .id(id)
        .debug_selector(move || selector)
        .flex_shrink_0()
        .flex()
        .items_center()
        .justify_center()
        .gap_2()
        .h(px(metrics::BUTTON_HEIGHT))
        .px_3()
        .rounded(px(metrics::BUTTON_RADIUS))
        .cursor_pointer()
        .text_size(px(13.))
        .font_weight(FontWeight::MEDIUM)
        .bg(rgb(if primary { c.accent } else { c.surface }))
        .text_color(rgb(if primary { c.on_accent } else { c.muted }))
        .hover(move |s| {
            s.bg(rgb(if primary {
                c.accent_hover
            } else if selected_hover {
                c.soft
            } else {
                c.raised
            }))
        })
        .active(|s| s.opacity(0.75))
}
/// Icon-only sizing; callers attach a descriptive tooltip.
pub fn icon_button(button: Stateful<Div>) -> Stateful<Div> {
    button.w(px(metrics::ICON_BUTTON)).px_0()
}
pub fn toolbar_button(button: Stateful<Div>) -> Stateful<Div> {
    button
        .h(px(metrics::TOOLBAR_BUTTON))
        .w(px(metrics::TOOLBAR_BUTTON))
        .rounded(px(7.))
        // Mark control input handled so AppKit does not begin a window drag
        // before GPUI can receive mouse-up and dispatch the click.
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
}
pub fn topbar(id: &'static str, c: Colors, border: bool) -> Stateful<Div> {
    div()
        .id(id)
        .debug_selector(move || id.into())
        .flex()
        .items_center()
        .h(px(metrics::TITLEBAR))
        .flex_shrink_0()
        .pl(px(if cfg!(target_os = "macos") { 80. } else { 12. }))
        .pr_3()
        .gap_1()
        .when(border, |d| d.border_b_1().border_color(rgb(c.border)))
        .on_click(|event, w, _| {
            if event.click_count() == 2 {
                w.titlebar_double_click();
            }
        })
}
/// The app's full-color logo and name, placed at the leading edge of the
/// topbar after the sidebar toggle. `logo` is an asset path drawn with `img`
/// on the shared 640×540 brand canvas (Picacho, Ironwood, Tonto).
pub fn app_brand(logo: &'static str, name: &'static str, show_name: bool) -> Div {
    div()
        .debug_selector(|| "app-brand".into())
        .flex()
        .items_center()
        .flex_shrink_0()
        .ml(px(6.))
        .child(img(logo).w(px(20.)).h(px(17.)))
        .when(show_name, |d| {
            d.child(
                div()
                    .ml_1()
                    .text_size(px(13.))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(name),
            )
        })
}
pub fn sidebar(c: Colors) -> Div {
    div()
        .h_full()
        .flex_shrink_0()
        .overflow_hidden()
        .flex()
        .flex_col()
        .border_r_1()
        .border_color(rgb(c.border))
}
pub fn split_panes(
    layout: crate::PaneLayout,
    list: impl IntoElement,
    detail: impl IntoElement,
) -> Div {
    div()
        .flex()
        .flex_1()
        .min_h_0()
        .min_w_0()
        .when(layout.show_list, |d| d.child(list))
        .when(layout.show_detail, |d| d.child(detail))
}
/// Window-bounded modal surface. Focus containment and dismissal belong to the host.
pub fn modal_panel(window: &Window, c: Colors, width: f32, height: Option<f32>) -> Stateful<Div> {
    let available = window
        .viewport_size()
        .map(|v| (v - px(metrics::DIALOG_MARGIN * 2.)).max(px(0.)));
    div()
        .id("modal-panel")
        .debug_selector(|| "modal-panel".into())
        .flex()
        .flex_col()
        .w(px(width))
        .max_w(available.width)
        .max_h(available.height)
        .when_some(height, |d, height| d.h(px(height).min(available.height)))
        .rounded(px(metrics::DIALOG_RADIUS))
        .border_1()
        .border_color(rgb(c.border))
        .bg(rgb(c.surface))
        .shadow_2xl()
        .overflow_hidden()
}
pub fn modal_backdrop() -> Div {
    div()
        .occlude()
        .absolute()
        .inset_0()
        .bg(rgba(0x101c2c66))
        .flex()
        .items_center()
        .justify_center()
        .p_4()
}
pub fn settings_narrow(window: &Window) -> bool {
    window.viewport_size().width < px(metrics::SETTINGS_BREAKPOINT)
}
pub fn settings_nav(c: Colors, narrow: bool) -> Stateful<Div> {
    div()
        .id("settings-nav")
        .debug_selector(|| "settings-nav".into())
        .w(px(metrics::SETTINGS_NAV))
        .flex_shrink_0()
        .h_full()
        .min_h_0()
        .overflow_y_scroll()
        .p_3()
        .flex()
        .flex_col()
        .gap(px(2.))
        .bg(rgb(c.bg))
        .border_r_1()
        .border_color(rgb(c.border))
        .when(narrow, |d| {
            d.w_full()
                .h(px(56.))
                .min_w_0()
                .flex_row()
                .items_center()
                .gap_2()
                .overflow_y_hidden()
                .overflow_x_scroll()
                .border_r_0()
                .border_b_1()
        })
}
pub fn settings_shell(narrow: bool) -> Div {
    div()
        .size_full()
        .min_h_0()
        .flex()
        .when(narrow, |d| d.flex_col())
}
pub fn settings_page(narrow: bool) -> Stateful<Div> {
    div()
        .id("settings-page")
        .debug_selector(|| "settings-page".into())
        .flex_1()
        .min_h_0()
        .min_w_0()
        .overflow_y_scroll()
        .p_6()
        .when(narrow, |d| d.p_4())
}
pub fn popup_surface(c: Colors) -> Div {
    div()
        .min_w(px(200.))
        .p_1()
        .flex()
        .flex_col()
        .rounded(px(10.))
        .border_1()
        .border_color(rgb(c.border))
        .bg(rgb(c.raised))
        .shadow_lg()
        .text_size(px(13.))
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .on_mouse_down(MouseButton::Right, |_, _, cx| cx.stop_propagation())
}
/// Draw a themed SVG without imposing an asset namespace on applications.
pub fn icon(path: impl Into<SharedString>, size: f32, color: u32) -> Div {
    div()
        .size(px(size))
        .flex_shrink_0()
        .child(svg().path(path.into()).size_full().text_color(rgb(color)))
}

/// Separate related toolbar controls without consuming excessive horizontal space.
pub fn toolbar_separator(c: Colors) -> Div {
    div()
        .w(px(1.))
        .h(px(18.))
        .mx_2()
        .flex_shrink_0()
        .bg(rgb(c.border))
}

pub fn menu_separator(c: Colors) -> Div {
    div().h(px(1.)).my_1().mx_2().bg(rgb(c.border))
}

/// A menu command surface. Hosts attach callbacks and own menu dismissal.
pub fn menu_item(
    id: impl Into<SharedString>,
    label: impl Into<SharedString>,
    shortcut: &str,
    c: Colors,
) -> Stateful<Div> {
    button(id, c, false, true)
        .w_full()
        .h(px(32.))
        .justify_start()
        .bg(transparent_black())
        .child(div().flex_1().child(label.into()))
        .child(
            div()
                .text_size(px(11.))
                .text_color(rgb(c.muted))
                .child(shortcut.to_owned()),
        )
}

/// Bound a pointer-anchored menu to the current window, including small windows.
pub fn pointer_menu(w: &Window, at: Point<Pixels>, c: Colors) -> Stateful<Div> {
    let viewport = w.viewport_size();
    let top =
        at.y.min((viewport.height - px(234.)).max(px(8.)))
            .max(px(8.));
    div()
        .id("object-menu")
        .debug_selector(|| "object-menu".into())
        .absolute()
        .left(
            at.x.min((viewport.width - px(244.)).max(px(8.)))
                .max(px(8.)),
        )
        .top(top)
        .w(px(236.))
        .max_w((viewport.width - px(16.)).max(px(0.)))
        .max_h((viewport.height - top - px(8.)).max(px(0.)))
        .overflow_y_scroll()
        .p_1()
        .rounded(px(10.))
        .border_1()
        .border_color(rgb(c.border))
        .bg(rgb(c.raised))
        .shadow_lg()
        .text_size(px(13.))
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .on_mouse_down(MouseButton::Right, |_, _, cx| cx.stop_propagation())
}
