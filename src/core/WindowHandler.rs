use std::num::NonZeroU32;

use glium::glutin;
use glium::glutin::config::ConfigTemplateBuilder;
use glium::glutin::context::ContextAttributesBuilder;
use glium::glutin::display::GetGlDisplay;
use glium::glutin::prelude::{GlDisplay, NotCurrentGlContext};
use glium::glutin::surface::{SurfaceAttributesBuilder, WindowSurface};
use raw_window_handle::HasWindowHandle;

use imgui_winit_support::winit::{dpi::LogicalSize, event_loop::EventLoop};
use winit::raw_window_handle;
use winit::window::{Window, WindowAttributes};




pub (crate) struct WindowHandler {
    title : String,
}

impl WindowHandler {

    pub fn init(title: &str) -> WindowHandler{
        WindowHandler{

            title :title.to_string(),
        }
    }
    pub fn create_window(&self) -> (EventLoop<()>, Window, glium::Display<WindowSurface>) {
        let event_loop = EventLoop::new().expect("Failed to create EventLoop");
    
        let window_attributes = WindowAttributes::default()
            .with_title(&self.title)
            .with_inner_size(LogicalSize::new(1024, 768));
    
        let (window, cfg) = glutin_winit::DisplayBuilder::new()
            .with_window_attributes(Some(window_attributes.clone()))
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
    
        let surface_attribs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            window.window_handle().unwrap().as_raw(),
            NonZeroU32::new(1024).unwrap(),
            NonZeroU32::new(768).unwrap(),
        );
        let surface = unsafe {
            cfg.display()
                .create_window_surface(&cfg, &surface_attribs)
                .expect("Failed to create OpenGL surface")
        };
    
        let context = context
            .make_current(&surface)
            .expect("Failed to make OpenGL context current");
    
        let display = glium::Display::from_context_surface(context, surface)
            .expect("Failed to create glium Display");
        
        (event_loop, window, display)
    } 
    
    pub fn imgui_init(&self , window: &Window) -> (imgui_winit_support::WinitPlatform, imgui::Context) {
        let mut imgui_context = imgui::Context::create();
        imgui_context.set_ini_filename(None);

        let mut winit_platform = imgui_winit_support::WinitPlatform::new(&mut imgui_context);

        let dpi_mode = imgui_winit_support::HiDpiMode::Default;

        winit_platform.attach_window(imgui_context.io_mut(), window, dpi_mode);

        imgui_context
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

        (winit_platform, imgui_context)
    }


}