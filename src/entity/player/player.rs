use glium::{self, glutin::surface::WindowSurface, implement_vertex, index::NoIndices, vertex::{self, VertexBufferAny}, Display, Program, Texture2d, Vertex, VertexBuffer};

struct Player_Data{
    
}
pub(crate) struct Player{
    _display:Display<WindowSurface>,
}

impl Player {
    
    pub fn init(_display:&Display<WindowSurface>) -> Self {
        Self{
            _display:_display.clone()
        }
    }
    
    pub fn load_entity(&mut self) -> (VertexBufferAny,NoIndices,Program,Texture2d) {
        
        #[derive(Clone,Copy)]
        struct vertex{
            position : [f32;2],
            texture_cordinate : [f32;2],
        }
        implement_vertex!(vertex,position,texture_cordinate);

        let vertex_shader_src = r#"
            #version 140

            in vec2 position;
            in vec2 texture_cordinate;
            out vec2 v_texture_cordinate;
            uniform mat4 matrix;

            void main() {
                v_texture_cordinate = texture_cordinate;
                gl_Position = matrix * vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            in vec2 v_texture_cordinate;
            out vec4 color;
            uniform sampler2D tex;

            void main() {
                color = texture(tex, v_texture_cordinate);
            }
        "#;


        let image = image::load(std::io::Cursor::new(&include_bytes!("./character.png")),
                                                        image::ImageFormat::Png).unwrap().to_rgba8();

        let image_dimantions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimantions);
        let texture = glium::texture::Texture2d::new(&self._display, image).unwrap();

        let shape = vec![
            vertex { position: [10.0 ,10.0 ] ,texture_cordinate:[1.0,1.0]},
            vertex { position: [-10.0,10.0 ] ,texture_cordinate:[0.0,1.0]},
            vertex { position: [-10.0,-10.0 ] ,texture_cordinate:[0.0,0.0]},
            vertex { position: [10.0,-10.0 ] ,texture_cordinate:[1.0,0.0]},

        ];

        let vertex_buffer: VertexBuffer<vertex> = glium::VertexBuffer::new(&self._display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

        let program = glium::Program::from_source(&self._display, &vertex_shader_src, &fragment_shader_src, None).unwrap();
        

        (vertex_buffer.into(),indices,program,texture)
    }
    
}