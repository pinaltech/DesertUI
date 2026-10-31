//! Picacho palettes and stable preference keys.
#[derive(Clone, Copy)]
pub struct Colors {
    pub bg: u32,
    pub surface: u32,
    pub raised: u32,
    pub text: u32,
    pub muted: u32,
    pub border: u32,
    pub accent: u32,
    pub accent_hover: u32,
    /// Text and icons drawn on the accent color.
    pub on_accent: u32,
    pub soft: u32,
    pub danger: u32,
}
impl Colors {
    pub fn new(dark: bool, accent: Accent) -> Self {
        let (accent, soft, accent_hover, on_accent) = accent.colors(dark);
        if dark {
            Self {
                bg: 0x1c1f26,
                surface: 0x242831,
                raised: 0x2a2f39,
                text: 0xedf0f5,
                muted: 0xa9b3c4,
                border: 0x3c4452,
                accent,
                accent_hover,
                on_accent,
                soft,
                danger: 0xff9aa7,
            }
        } else {
            Self {
                bg: 0xeef0f3,
                surface: 0xf4f5f7,
                raised: 0xf8f9fb,
                text: 0x242b36,
                muted: 0x586474,
                border: 0xd8dde5,
                accent,
                accent_hover,
                on_accent,
                soft,
                danger: 0xa8313e,
            }
        }
    }
}
/// The theme color chosen in Settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accent {
    Green,
    Teal,
    Blue,
    Purple,
    Pink,
    Orange,
    Graphite,
}
impl Accent {
    pub const ALL: [Self; 7] = [
        Self::Green,
        Self::Teal,
        Self::Blue,
        Self::Purple,
        Self::Pink,
        Self::Orange,
        Self::Graphite,
    ];
    pub fn key(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Teal => "teal",
            Self::Blue => "blue",
            Self::Purple => "purple",
            Self::Pink => "pink",
            Self::Orange => "orange",
            Self::Graphite => "graphite",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Green => "Green",
            Self::Teal => "Teal",
            Self::Blue => "Blue",
            Self::Purple => "Purple",
            Self::Pink => "Pink",
            Self::Orange => "Orange",
            Self::Graphite => "Graphite",
        }
    }
    pub fn from_key(key: &str) -> Self {
        Self::ALL
            .into_iter()
            .find(|a| a.key() == key)
            .unwrap_or(Self::Green)
    }
    /// (accent, soft, hover, on accent). Light accents keep white text above
    /// 4.5:1 contrast; dark accents are light enough for dark text.
    pub fn colors(self, dark: bool) -> (u32, u32, u32, u32) {
        match (self, dark) {
            (Self::Green, false) => (0x246846, 0xdcebe2, 0x2a7650, 0xffffff),
            (Self::Green, true) => (0x86cfaa, 0x30463b, 0x98ddba, 0x13291e),
            (Self::Teal, false) => (0x0b6b70, 0xd6ecec, 0x0f7a80, 0xffffff),
            (Self::Teal, true) => (0x7fd0d0, 0x2b4647, 0x93dcdc, 0x0f2a2b),
            (Self::Blue, false) => (0x2459a8, 0xdde7f6, 0x2c66bd, 0xffffff),
            (Self::Blue, true) => (0x93b8f0, 0x2e3a52, 0xa7c6f4, 0x132440),
            (Self::Purple, false) => (0x6741a8, 0xe8e0f5, 0x7550b8, 0xffffff),
            (Self::Purple, true) => (0xbea5ef, 0x3e3557, 0xcbb6f3, 0x261a40),
            (Self::Pink, false) => (0xa8336b, 0xf5dfe9, 0xb83f79, 0xffffff),
            (Self::Pink, true) => (0xf0a3c6, 0x53334a, 0xf4b5d2, 0x3d1428),
            (Self::Orange, false) => (0xa64a12, 0xf6e3d6, 0xb85716, 0xffffff),
            (Self::Orange, true) => (0xf2b07f, 0x553c2c, 0xf5bf95, 0x3d2210),
            (Self::Graphite, false) => (0x444c59, 0xe3e6eb, 0x525b69, 0xffffff),
            (Self::Graphite, true) => (0xc2c9d4, 0x3a404b, 0xd1d7e0, 0x1d2229),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appearance {
    System,
    Light,
    Dark,
}
impl Appearance {
    pub fn key(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}
