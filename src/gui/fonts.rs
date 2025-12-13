use imgui::FontId;

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