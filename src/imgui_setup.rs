use imgui::{FontId};
use imgui_winit_support::{winit::window::Window, WinitPlatform};
use crate::util::roboto_font;
use crate::fonts::Fonts;

pub fn imgui_init(window: &Window) -> (WinitPlatform, imgui::Context, Fonts) {
    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(None);

    let mut winit_platform = WinitPlatform::new(&mut imgui_context);
    winit_platform.attach_window(
        imgui_context.io_mut(),
        window,
        imgui_winit_support::HiDpiMode::Rounded,
    );

    let main_font: FontId = imgui_context
        .fonts()
        .add_font(&[roboto_font(26.0)]);
    let title_font: FontId = imgui_context
        .fonts()
        .add_font(&[roboto_font(94.0)]);
    let header_font: FontId = imgui_context
        .fonts()
        .add_font(&[roboto_font(64.0)]);
    let big_font: FontId = imgui_context
        .fonts()
        .add_font(&[roboto_font(68.0)]);
    let medium_font: FontId = imgui_context
        .fonts()
        .add_font(&[roboto_font(48.0)]);

    let fonts: Fonts = Fonts::new(main_font, title_font, header_font, big_font, medium_font);
    imgui_context.style_mut().frame_padding = [4.0, 6.0];
    imgui_context.style_mut().item_spacing = [8.0, 5.0];
    imgui_context.style_mut().frame_rounding = 4.0;
    imgui_context.io_mut().font_global_scale = (1.0 / winit_platform.hidpi_factor()) as f32;

    (winit_platform, imgui_context, fonts)
}