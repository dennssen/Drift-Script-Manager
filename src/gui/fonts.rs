use imgui::{FontConfig, FontGlyphRanges, FontId, FontSource};

pub const ROBOTO_PATH: &[u8] = include_bytes!("../../assets/fonts/Roboto-VariableFont_wdth,wght.ttf");
pub const ICON_PATH: &[u8] = include_bytes!("../../assets/fonts/FontAwesome.otf");

pub const MAIN_SIZE: f32 = 26.0;
pub const TITLE_SIZE: f32 = 94.0;
pub const HEADER_SIZE: f32 = 64.0;
pub const BIG_SIZE: f32 = 68.0;
pub const MEDIUM_SIZE: f32 = 48.0;

pub struct Fonts {
    pub main_font: FontId,
    pub title_font: FontId,
    pub header_font: FontId,
    pub big_font: FontId,
    pub medium_font: FontId,
}

impl Fonts {
    pub fn new(
        main_font: FontId,
        title_font: FontId,
        header_font: FontId,
        big_font: FontId,
        medium_font: FontId,
    ) -> Self {
        Self {
            main_font,
            title_font,
            header_font,
            big_font,
            medium_font,
        }
    }
}

pub fn roboto_font(font_size: f32) -> FontSource<'static> {
    FontSource::TtfData {
        data: ROBOTO_PATH,
        size_pixels: font_size,
        config: None
    }
}

pub fn icon_font(size: f32) -> FontSource<'static> {
    FontSource::TtfData {
        data: ICON_PATH,
        size_pixels: size,
        config: Some(FontConfig {
            glyph_ranges: FontGlyphRanges::from_slice(&[0xf000, 0xf3ff, 0]),
            ..Default::default()
        }),
    }
}