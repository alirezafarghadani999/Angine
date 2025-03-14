
use std::env;

use glium::winit::event::{ElementState, Event, KeyEvent, MouseButton, WindowEvent};
use glium::winit::keyboard::Key;
use glium::winit::{self, event, keyboard};
use glium::{self, glutin, implement_vertex, uniform, Blend, VertexBuffer};
use glium::{glutin::api::egl::display, winit::{dpi::Size, event_loop, window}, Surface};

use imgui::{Context, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};


#[path="entity/player/player.rs"]
mod player;
use player::Player;

#[path="core/WindowHandler.rs"]
mod WindowHandler;


#[path="core/EnvirementLogic.rs"]
mod EnvirementLogic;


fn round_to_two_decimal_places(value: f32) -> f32 {
    (value).round()
}

fn main(){

let env = EnvirementLogic::EnvirementLogic::create(20,0,5f32);

let mut map= env.get_env(); 
let mut map_bk = map.clone();

let mut hide = false;

let winhan = WindowHandler::WindowHandler::init( "test");
let (event_loop, _window, _display) = winhan.create_window();

let (mut winit_platform, mut imgui_context) = winhan.imgui_init(&_window);

// Create renderer from this crate
let mut renderer = imgui_glium_renderer::Renderer::new(&mut imgui_context, &_display)
    .expect("Failed to initialize renderer");

// Timer for FPS calculation
let mut last_frame = std::time::Instant::now();


let mut player = Player::init(&_display);
player.load_entity();

let (vertex_buffer,indices,program,texture) = player::Player::init(&_display).load_entity();

#[derive(Clone,Copy)]
struct vertex{
    position : [f32;2],
                }
implement_vertex!(vertex,position);

let mut _window_size : (u32,u32) = (400,400);

let mut mouse_x = 0f32;
let mut mouse_y = 0f32;

let mut target_x = 0f32;
let mut target_y = 0f32;

let mut x = 0f32;
let mut y = 0f32;

let mut qx:f32 = 0.0;
let mut qy:f32 = 0.0;

let mut can_move = false;
let mut in_ui = false;
let step_size = 2f32;

// event loop
#[allow(deprecated)]
let _ = event_loop.run(move | event , window_target |  {
    winit_platform.handle_event(imgui_context.io_mut(), &_window, &event);

    match event {


        glium::winit::event::Event::WindowEvent { window_id, event } => match event {
            
            glium::winit::event::WindowEvent::CloseRequested => window_target.exit() ,

            glium::winit::event::WindowEvent::Resized(new_size)=> {
                
                if new_size.width > 0 && new_size.height > 0 {
                    _window_size = new_size.into();
                    _display.resize((new_size.width, new_size.height));
                }
            }
            glium::winit::event::WindowEvent::RedrawRequested =>{




                 let vertex_shader_src2 = r#"
                     #version 140

                     in vec2 position;
                     uniform mat4 matrix;

                     void main() {
                         gl_Position = matrix * vec4(position, 0.0, 1.0);
                     }
                 "#;

                 let fragment_shader_src2 = r#"
                     #version 140

                     out vec4 color;
                     uniform sampler2D tex;
                     uniform float c;

                     void main() {
                         color = vec4(c, c ,c, 1.0);
                     }
                 "#;
                
                 let shape2 = vec![
                     vertex { position: [env.get_tileSize() ,env.get_tileSize() ] },
                     vertex { position: [-env.get_tileSize(),env.get_tileSize() ] },
                     vertex { position: [-env.get_tileSize(),-env.get_tileSize() ] },
                     vertex { position: [env.get_tileSize(),-env.get_tileSize() ] },

                 ];

                 let vertex_buffer2: VertexBuffer<vertex> = glium::VertexBuffer::new(&_display, &shape2).unwrap();
                 let indices2 = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

                 let program2 = glium::Program::from_source(&_display, &vertex_shader_src2, &fragment_shader_src2, None).unwrap();

                if can_move
                {
                    if (-step_size < x-target_x && x-target_x < step_size) && (-step_size < y-target_y && y-target_y < step_size) {can_move =false;}
                    else {
                        if x-target_x < 0f32 {x += step_size;}
                        if x-target_x > 0f32 {x -= step_size;}
                        if y-target_y < 0f32 {y += step_size;}
                        if y-target_y > 0f32 {y -= step_size;}
                    }  
                }
                qx = _window_size.0 as f32 / 2.0  ;
                qy = _window_size.1 as f32 / 2.0  ;


                let ui = imgui_context.frame();
                in_ui = ui.is_window_focused() || ui.is_any_item_focused() || ui.is_item_hovered() ;

                ui.text("Test1");
                ui.button_with_size("Alireza test btn", [100f32,50f32]);

                let mut target = _display.draw();

                target.clear_color(0f32,0f32,0f32, 1.0);


                for (keyi,i) in map.iter().enumerate() {
                    for (keyj,j) in i.iter().enumerate() {
                         target.draw(&vertex_buffer2, &indices2, &program2, &uniform! {
                                 matrix: [
                                 [(1.0 / qx*2.0), 0.0, 0.0, 0.0],
                                 [0.0, (1.0 / qy*2.0), 0.0, 0.0],
                                 [0.0, 0.0, 1.0, 0.0],
                                 [ (keyi as i32 - env.get_mapSize() as i32 /2 )
                                                         as f32 *env.get_tileSize() / qx*4.0 
                                , ( keyj as i32 - env.get_mapSize() as i32 /2)
                                                         as f32 *env.get_tileSize() / qy*4.0
                                , 0.0, 1.0f32],
                                 ] ,
                         c:(*j).abs()}, &Default::default()).unwrap();
                    }
                }

                target.draw(&vertex_buffer, &indices, &program, &uniform! {
                    matrix: [
                    [(1.0 / qx*2.0), 0.0, 0.0, 0.0],
                    [0.0, (1.0 / qy*2.0), 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [ (x)/ qx , (y) / qy , 0.0, 1.0f32],
                    ] , tex: &texture}, &glium::DrawParameters {
                        blend: Blend::alpha_blending(), 
                        ..Default::default()
                    }).unwrap();


                    winit_platform.prepare_render(ui, &_window);
                    let draw_data = imgui_context.render();
                    renderer
                        .render(&mut target, draw_data)
                        .expect("Rendering failed");
                
                target.finish().unwrap();
                
            },
            glium::winit::event::WindowEvent::MouseInput { button, state, .. } => {
                if button == MouseButton::Left && state == ElementState::Pressed {
                    if !in_ui{ 
                        can_move =true;
                        target_x = round_to_two_decimal_places(mouse_x - qx);
                        target_y = round_to_two_decimal_places(mouse_y + qy);
                    }

                }
            },
            glium::winit::event::WindowEvent::CursorMoved { position, .. } => {
                    mouse_x = (position.x as f32 ) ;
                    mouse_y = -(position.y as f32 );
            },
            _ => (),
        },
        glium::winit::event::Event::AboutToWait => {
            winit_platform
            .prepare_frame(imgui_context.io_mut(), &_window)
            .expect("Failed to prepare frame");
            _window.request_redraw();
        },
        glium::winit::event::Event::NewEvents(_) => {
            let now = std::time::Instant::now();
            imgui_context.io_mut().update_delta_time(now - last_frame);
            last_frame = now;
        },

        _ => (),
    }

});


}

