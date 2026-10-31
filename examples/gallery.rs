//! Small real application demonstrating composition without app-specific state.
use desert_ui::{self as ui, Accent, Colors, LayoutPolicy};
use gpui::{prelude::*, *};
use gpui_component::Root;

struct Gallery {
    selected: bool,
    settings: bool,
}
impl Render for Gallery {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        ui::native_titlebar::sync_layout(w);
        let c = Colors::new(ui::system_dark(w), Accent::Green);
        let layout =
            LayoutPolicy::default().resolve(w.viewport_size().width.into(), self.selected, false);
        div()
            .size_full()
            .relative()
            .flex()
            .flex_col()
            .bg(rgb(c.surface))
            .text_color(rgb(c.text))
            .child(
                ui::topbar("topbar", c, true)
                    .child("DesertUI")
                    .child(div().flex_1())
                    .child(
                        ui::toolbar_button(ui::button("settings", c, false, false))
                            .child("⚙")
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
                                .w(px(layout.sidebar_width))
                                .p_3()
                                .child("All"),
                        )
                    })
                    .child(ui::split_panes(
                        layout,
                        div()
                            .h_full()
                            .when_some(layout.list_width, |d, width| d.w(px(width)))
                            .when(layout.single, |d| d.flex_1())
                            .p_4()
                            .child(
                                ui::button("item", c, false, false)
                                    .child("Example item")
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.selected = true;
                                        cx.notify();
                                    })),
                            ),
                        div()
                            .flex_1()
                            .min_w_0()
                            .p_4()
                            .child(ui::button("back", c, false, false).child("Back").on_click(
                                cx.listener(|this, _, _, cx| {
                                    this.selected = false;
                                    cx.notify();
                                }),
                            ))
                            .child("Resize the window to explore the panes."),
                    )),
            )
            .when(self.settings, |d| {
                let narrow = ui::settings_narrow(w);
                d.child(
                    ui::modal_backdrop().child(
                        ui::modal_panel(w, c, 820., Some(600.))
                            .child(ui::button("close", c, false, false).child("Done").on_click(
                                cx.listener(|this, _, _, cx| {
                                    this.settings = false;
                                    cx.notify();
                                }),
                            ))
                            .child(
                                ui::settings_shell(narrow)
                                    .child(ui::settings_nav(c, narrow).child("General"))
                                    .child(
                                        ui::settings_page(narrow).child(
                                            "Settings content stays owned by the application.",
                                        ),
                                    ),
                            ),
                    ),
                )
            })
    }
}
fn main() {
    Application::new().run(|cx| {
        ui::init(cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1280.), px(820.)),
                    cx,
                ))),
                window_min_size: Some(size(px(360.), px(320.))),
                titlebar: Some(TitlebarOptions {
                    title: Some("DesertUI Gallery".into()),
                    appears_transparent: cfg!(target_os = "macos"),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |w, cx| {
                ui::native_titlebar::install(w);
                let view = cx.new(|_| Gallery {
                    selected: false,
                    settings: false,
                });
                cx.new(|cx| Root::new(view, w, cx))
            },
        )
        .unwrap();
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
        cx.activate(true);
    });
}
