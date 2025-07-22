use miniquad::*;

pub use glam::{vec2, vec3, vec4, Vec2, Vec3, Vec4};

mod shader;

pub mod app;
pub use app::*;

pub mod module;
pub use module::*;

pub mod object;
pub use object::*;

pub mod draw;
pub use draw::*;

pub mod shape;
pub use shape::*;


/*

Todo:
[#] Создание окна
[#] Основная логика
[#] Модули
[#] Логика модулей
[#] Добавление 2д объектов
[#] Изменение 2д объектов
[?] Добавить update и/или draw в objects
[?] Рисование 2д объектов
[] Добавление 3д объектов
[] Изменение 3д объектов
[] Рисование 3д объектов
[] Модели
[] Свет
[] Аудио
[] Добавление изображения
[] Добавление текста
[] Сохранение данных
[] Создание TGR-CLI

Структура:
lib - связь между устройством и движком {
    module - модули
    shader - шейдеры
    app - состояние движка {
        audio
        data
        object
        shape
        model
        resource
    }
}
*/


pub struct Engine {
    app: App,
    modules: Modules,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            app: App::default(),
            modules: Modules::default(),
        }
    }

    pub fn module(mut self, module: impl Module) -> Self {
        self.modules.add_module(&self.app, module);
        self
    }

    pub fn run(self, title: &str) {
        let conf = conf::Conf {
            window_title: title.to_string(),
            window_resizable: true,
            fullscreen: false,
            high_dpi: true,

            // Msaa
            #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
            sample_count: 4,

            #[cfg(any(target_os = "android", target_os = "ios"))]
            sample_count: 2,

            #[cfg(target_arch = "wasm32")]
            sample_count: 1,
            
            ..Default::default()
        };

        start(conf, || Box::new(Render::new(self.app, self.modules)));
    }
}


struct Render {
    // State
    app: App,
    modules: Modules,

    // Render
    ctx: Box<dyn RenderingBackend>,
    pipeline: Pipeline,
    bindings: Bindings,
}

impl Render {
    pub fn new(app: App, modules: Modules) -> Self {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();


        let vertices: [Vertex; 3] = [
            Vertex { pos : [ -0.5, -0.5 ], color: [1., 0., 0., 1.] },
            Vertex { pos : [  0.5, -0.5 ], color: [0., 1., 0., 1.] },
            Vertex { pos : [  0.0,  0.5 ], color: [0., 0., 1., 1.] },
        ];
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let indices: [u16; 3] = [0, 1, 2];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };


        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                        vertex: shader::VERTEX,
                        fragment: shader::FRAGMENT,
                    },
                shader::meta(),
            )
            .expect("Error to load shaders");

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_color", VertexFormat::Float4),
            ],
            shader,
            PipelineParams {
                depth_test: Comparison::Always,
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                alpha_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Zero,
                    BlendFactor::One,
                )),
                ..Default::default()
            },
        );


        Self {
            app,
            modules,
            ctx,
            pipeline,
            bindings,
        }
    }
}

impl EventHandler for Render {
    fn update(&mut self) {
        self.app = self.modules.update(&mut self.app);  
    }

    fn draw(&mut self) {
        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx.draw(0, 3, 1);
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}


#[repr(C)]
struct Vertex {
    pos: [f32; 2],
    color: [f32; 4],
}