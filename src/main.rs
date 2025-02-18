use glium::winit::event::{ElementState, Event, KeyEvent, MouseButton, WindowEvent};
use glium::winit::keyboard::Key;
use glium::winit::{self, event, keyboard};
use glium::{self, implement_vertex, uniform, Blend, VertexBuffer};
use glium::{glutin::api::egl::display, winit::{dpi::Size, event_loop, window}, Surface};


#[path="entity/player/player.rs"]
mod player;
use player::Player;

fn round_to_two_decimal_places(value: f32) -> f32 {
    (value).round()
}

fn main(){
 
let map = vec![

    // [0f32,-45f32,200f32,0f32],
    // [11f32,45f32,-40f32,6f32],
    // [0f32,-45f32,200f32,8f32],
    // [67f32,23f32,-440f32,99f32],
    [-420f32,-80f32,45f32]

    ];

let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().unwrap();
let (_window , _display ) = glium::backend::glutin::SimpleWindowBuilder::new()
.with_title("test")
.build(&event_loop);

let mut player = Player::init(&_display);
player.load_entity();

let (vertex_buffer,indices,program,texture) = player::Player::init(&_display).load_entity();

#[derive(Clone,Copy)]
struct vertex{
    position : [f32;2],
                }
implement_vertex!(vertex,position);

let mut border : f32 = 30.0;
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
let step_size = 2f32;

let _ = event_loop.run(move | event , window_target |  {
    match event {
        

        glium::winit::event::Event::WindowEvent { window_id, event } => match event {
            
            glium::winit::event::WindowEvent::CloseRequested => window_target.exit() ,
            glium::winit::event::WindowEvent::Resized(window_size) =>{
                _window_size = window_size.into();
                _display.resize(window_size.into());
            },
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
                         color = vec4(c, 0.0 ,0.0, 1.0);
                     }
                 "#;
                
                 let shape2 = vec![
                     vertex { position: [border ,border ] },
                     vertex { position: [-border,border ] },
                     vertex { position: [-border,-border ] },
                     vertex { position: [border,-border ] },

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


                let mut target = _display.draw();

                target.clear_color(0.47, 0.26, 0.17, 1.0);

                for (keyi,i) in map.iter().enumerate() {
                    for (keyj,j) in i.iter().enumerate() {
                         target.draw(&vertex_buffer2, &indices2, &program2, &uniform! {
                                 matrix: [
                                 [(1.0 / qx*2.0), 0.0, 0.0, 0.0],
                                 [0.0, (1.0 / qy*2.0), 0.0, 0.0],
                                 [0.0, 0.0, 1.0, 0.0],
                                 [ ((keyi as f32 )+(keyj as f32 ))*50.0 / qx , *j / qy , 0.0, 1.0f32],
                                 ] ,
                         c:(*j/500f32).abs()}, &Default::default()).unwrap();
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
                
                target.finish().unwrap();
                
            },
            glium::winit::event::WindowEvent::MouseInput { button, state, .. } => {
                if button == MouseButton::Left && state == ElementState::Pressed {
                    can_move =true;
                    target_x = round_to_two_decimal_places(mouse_x - qx);
                    target_y = round_to_two_decimal_places(mouse_y + qy);

                }
            },
            glium::winit::event::WindowEvent::CursorMoved { position, .. } => {
                // if (!can_move) {
                    
                    mouse_x = (position.x as f32 ) ;
                    mouse_y = -(position.y as f32 );
                // }

            },
            _ => (),
        },
        glium::winit::event::Event::AboutToWait => {
            _window.request_redraw();
        },
        _ => (),
    }

});


}
