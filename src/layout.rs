use crate::metrics;

/// Responsive application policy. Defaults preserve Picacho's three-pane UI.
#[derive(Clone, Copy, Debug)]
pub struct LayoutPolicy {
    pub single_below: f32,
    pub rail_below: f32,
    pub list_width: f32,
    pub compact_list_width: f32,
    /// Ironwood supplies navigation in a bottom bar on all mobile pages.
    pub mobile_navigation: bool,
}
impl Default for LayoutPolicy {
    fn default() -> Self {
        Self {
            single_below: 900.,
            rail_below: 1180.,
            list_width: 380.,
            compact_list_width: 320.,
            mobile_navigation: false,
        }
    }
}
impl LayoutPolicy {
    /// Preserve Ironwood's existing responsive behavior during incremental migration.
    pub const IRONWOOD: Self = Self {
        single_below: 780.,
        rail_below: 1000.,
        list_width: 330.,
        compact_list_width: 330.,
        mobile_navigation: true,
    };
    pub fn resolve(self, width: f32, selected: bool, collapsed: bool) -> PaneLayout {
        let single = width < self.single_below;
        let compact = width < self.rail_below;
        PaneLayout {
            single,
            compact,
            show_sidebar: width >= metrics::MOBILE_BREAKPOINT
                || !(self.mobile_navigation || single && selected),
            sidebar_width: if compact || collapsed {
                metrics::SIDEBAR_RAIL
            } else {
                metrics::SIDEBAR
            },
            show_list: !single || !selected,
            show_detail: !single || selected,
            list_width: if single {
                None
            } else if compact {
                Some(self.compact_list_width)
            } else {
                Some(self.list_width)
            },
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PaneLayout {
    pub single: bool,
    pub compact: bool,
    pub show_sidebar: bool,
    pub sidebar_width: f32,
    pub show_list: bool,
    pub show_detail: bool,
    pub list_width: Option<f32>,
}
