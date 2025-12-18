use std::num::NonZeroU32;
use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use imgui::FontId;
use imgui_winit_support::WinitPlatform;
use raw_window_handle::HasWindowHandle;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Icon, Window, WindowAttributes, WindowButtons};
use crate::gui::fonts;
use crate::gui::fonts::Fonts;
use crate::utils::ui_helpers::roboto_font;

#[cfg(target_os = "windows")]
const ICON_256: &[u8] = include_bytes!("../../assets/logo/256.png");
const ICON_32: &[u8] = include_bytes!("../../assets/logo/32.png");
const TITLE: &str = "Drift Script Manager";
const INITIAL_WIDTH: u32 = 1024;
const INITIAL_HEIGHT: u32 = 768;

pub fn create_window() -> (
    EventLoop<()>,
    Window,
    Surface<WindowSurface>,
    PossiblyCurrentContext,
) {
    let event_loop = EventLoop::new().unwrap();

    let window_icon = load_icon(ICON_32);

    #[cfg(target_os = "windows")]
    let taskbar_icon = load_icon(ICON_256);
    #[cfg(target_os = "windows")]
    let window_attributes = WindowAttributes::default()
        .with_title(TITLE)
        .with_window_icon(window_icon)
        .with_taskbar_icon(taskbar_icon)
        .with_inner_size(LogicalSize::new(1024, 768))
        .with_resizable(false)
        .with_maximized(false)
        .with_enabled_buttons(WindowButtons::all() & !WindowButtons::MAXIMIZE);

    #[cfg(target_os = "linux")]
    let window_attributes = WindowAttributes::default()
        .with_title(TITLE)
        .with_window_icon(window_icon)
        .with_inner_size(LogicalSize::new(1024, 768))
        .with_resizable(false)
        .with_maximized(false)
        .with_enabled_buttons(WindowButtons::all() & !WindowButtons::MAXIMIZE);
    let (window, cfg) = glutin_winit::DisplayBuilder::new()
        .with_window_attributes(Some(window_attributes))
        .build(&event_loop, ConfigTemplateBuilder::new(), |mut configs| {
            configs.next().unwrap()
        })
        .expect("Failed to create OpenGL window");

    let window = window.unwrap();

    let context_attribs =
        ContextAttributesBuilder::new().build(Some(window.window_handle().unwrap().as_raw()));
    let context = unsafe {
        cfg.display()
            .create_context(&cfg, &context_attribs)
            .expect("Failed to create OpenGL context")
    };

    let surface_attribs = SurfaceAttributesBuilder::<WindowSurface>::new()
        .with_srgb(Some(true))
        .build(
            window.window_handle().unwrap().as_raw(),
            NonZeroU32::new(INITIAL_WIDTH).unwrap(),
            NonZeroU32::new(INITIAL_HEIGHT).unwrap(),
        );
    let surface = unsafe {
        cfg.display()
            .create_window_surface(&cfg, &surface_attribs)
            .expect("Failed to create OpenGL surface")
    };

    let context = context
        .make_current(&surface)
        .expect("Failed to make OpenGL context current");

    (event_loop, window, surface, context)
}

pub fn glow_context(context: &PossiblyCurrentContext) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function_cstr(|s| context.display().get_proc_address(s).cast())
    }
}

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
        .add_font(&[roboto_font(fonts::MAIN_SIZE)]);
    let title_font: FontId = imgui_context
        .fonts()
        .add_font(&[roboto_font(fonts::TITLE_SIZE)]);
    let header_font: FontId = imgui_context
        .fonts()
        .add_font(&[roboto_font(fonts::HEADER_SIZE)]);
    let big_font: FontId = imgui_context
        .fonts()
        .add_font(&[roboto_font(fonts::BIG_SIZE)]);
    let medium_font: FontId = imgui_context
        .fonts()
        .add_font(&[roboto_font(fonts::MEDIUM_SIZE)]);

    let fonts: Fonts = Fonts::new(main_font, title_font, header_font, big_font, medium_font);
    imgui_context.style_mut().frame_padding = [4.0, 6.0];
    imgui_context.style_mut().item_spacing = [8.0, 5.0];
    imgui_context.style_mut().frame_rounding = 4.0;
    imgui_context.io_mut().font_global_scale = (1.0 / winit_platform.hidpi_factor()) as f32;

    (winit_platform, imgui_context, fonts)
}

fn load_icon(bytes: &[u8]) -> Option<Icon> {
    let image = image::load_from_memory(bytes).ok()?.into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    Icon::from_rgba(rgba, width, height).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icons_are_valid() {
        assert!(load_icon(ICON_32).is_some(), "32px icon failed to load");

        #[cfg(target_os = "windows")]
        assert!(load_icon(ICON_256).is_some(), "256px icon failed to load");
    }
}