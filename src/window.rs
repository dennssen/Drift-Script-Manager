use std::num::NonZeroU32;

use glutin::{
    config::ConfigTemplateBuilder,
    context::{NotCurrentGlContext, ContextAttributesBuilder, PossiblyCurrentContext},
    display::{GlDisplay, GetGlDisplay},
    surface::{Surface, SurfaceAttributesBuilder, WindowSurface},
};
use imgui_winit_support::winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
};
use raw_window_handle::HasWindowHandle;
use winit::window::WindowButtons;
use crate::util::{load_icon, ICON_256, ICON_32};

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