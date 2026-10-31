use desert_ui::{self as ui, Accent, Colors, LayoutPolicy};
use gpui::{prelude::*, *};
use std::prelude::v1::test;

struct Harness {
    selected: bool,
    settings: bool,
    activations: usize,
}
impl Render for Harness {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = Colors::new(false, Accent::Green);
        let layout = LayoutPolicy::default().resolve(
            window.viewport_size().width.into(),
            self.selected,
            false,
        );
        let list = div()
            .id("list")
            .debug_selector(|| "list".into())
            .h_full()
            .when_some(layout.list_width, |d, width| d.w(px(width)).flex_shrink_0())
            .when(layout.list_width.is_none(), |d| d.flex_1().min_w_0())
            .child(
                ui::button("select", c, true, false)
                    .child("Select")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.selected = true;
                        this.activations += 1;
                        cx.notify();
                    })),
            );
        let detail = div()
            .id("detail")
            .debug_selector(|| "detail".into())
            .flex_1()
            .min_w_0()
            .child(
                ui::button("back", c, false, false)
                    .child("Back")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.selected = false;
                        cx.notify();
                    })),
            );
        div()
            .size_full()
            .relative()
            .flex()
            .flex_col()
            .child(
                ui::topbar("topbar", c, true).child(
                    ui::toolbar_button(ui::button("open-settings", c, false, false))
                        .child("Settings")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.settings = true;
                            cx.notify();
                        })),
                ),
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .min_h_0()
                    .when(layout.show_sidebar, |d| {
                        d.child(
                            ui::sidebar(c)
                                .id("sidebar")
                                .debug_selector(|| "sidebar".into())
                                .w(px(layout.sidebar_width)),
                        )
                    })
                    .child(ui::split_panes(layout, list, detail)),
            )
            .when(self.settings, |d| {
                let narrow = ui::settings_narrow(window);
                d.child(
                    ui::modal_backdrop().child(
                        ui::modal_panel(window, c, 820., Some(600.))
                            .child(div().h(px(48.)).flex_shrink_0().child("Settings"))
                            .child(
                                ui::settings_shell(narrow)
                                    .child(ui::settings_nav(c, narrow).child("General"))
                                    .child(
                                        ui::settings_page(narrow)
                                            .child(div().h(px(1200.)).child("Scrollable content")),
                                    ),
                            ),
                    ),
                )
            })
    }
}
fn frame(cx: &mut VisualTestContext) {
    cx.simulate_event(MouseMoveEvent {
        position: point(px(1.), px(1.)),
        pressed_button: None,
        modifiers: Modifiers::none(),
    });
    cx.run_until_parked();
}
fn bounds(cx: &mut VisualTestContext, name: &'static str) -> Bounds<Pixels> {
    cx.debug_bounds(name)
        .unwrap_or_else(|| panic!("Missing {name}"))
}
fn inside(b: Bounds<Pixels>, width: f32, height: f32) {
    assert!(
        b.left() >= px(0.)
            && b.right() <= px(width)
            && b.top() >= px(0.)
            && b.bottom() <= px(height),
        "{b:?} outside {width}x{height}"
    );
}
#[gpui::test]
fn real_layout_and_clicks_keep_list_detail_and_settings_reachable(cx: &mut TestAppContext) {
    cx.update(ui::init);
    let mut view = None;
    let window = cx.add_window(|w, cx| {
        let v = cx.new(|_| Harness {
            selected: false,
            settings: false,
            activations: 0,
        });
        view = Some(v.clone());
        gpui_component::Root::new(v, w, cx)
    });
    let view = view.unwrap();
    let cx = VisualTestContext::from_window(window.into(), cx).into_mut();
    for (width, height) in [
        (1440., 1000.),
        (1180., 700.),
        (960., 600.),
        (899., 600.),
        (700., 480.),
        (640., 480.),
        (480., 420.),
        (360., 320.),
    ] {
        cx.simulate_resize(size(px(width), px(height)));
        frame(cx);
        inside(bounds(cx, "list"), width, height);
        assert_eq!(bounds(cx, "topbar").size.height, px(38.));
        let select = bounds(cx, "select");
        cx.simulate_click(select.center(), Modifiers::none());
        frame(cx);
        view.read_with(cx, |v, _| assert!(v.selected));
        inside(bounds(cx, "detail"), width, height);
        if width < 900. {
            let detail = bounds(cx, "detail");
            let expected_left = if width < 480. { 0. } else { 60. };
            assert_eq!(detail.left(), px(expected_left));
            assert_eq!(detail.right(), px(width));
        }
        let back = bounds(cx, "back");
        cx.simulate_click(back.center(), Modifiers::none());
        frame(cx);
        inside(bounds(cx, "list"), width, height);
        let settings = bounds(cx, "open-settings");
        cx.simulate_click(settings.center(), Modifiers::none());
        frame(cx);
        view.read_with(cx, |v, _| assert!(v.settings));
        for name in ["modal-panel", "settings-nav", "settings-page"] {
            inside(bounds(cx, name), width, height);
        }
        let panel = bounds(cx, "modal-panel");
        assert!(panel.size.width <= px(width - 32.));
        assert!(panel.size.height <= px(height - 32.));
        let nav = bounds(cx, "settings-nav");
        let page = bounds(cx, "settings-page");
        if width < 700. {
            assert!(nav.bottom() <= page.top());
            assert_eq!(nav.size.height, px(56.));
        } else {
            assert!(nav.right() <= page.left());
            assert_eq!(nav.size.width, px(210.));
        }
        view.update_in(cx, |v, _, cx| {
            v.settings = false;
            cx.notify();
        });
    }
    view.read_with(cx, |v, _| assert_eq!(v.activations, 8));
}
