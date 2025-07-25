use glam::Mat4;
use miniquad::*;

pub use glam::{vec2, vec3, vec4, Vec2, Vec3, Vec4};

mod shader;

pub mod app;
pub use app::*;

pub mod module;
pub use module::*;

pub mod object2d;
pub use object2d::*;

pub mod camera2d;
pub use camera2d::*;

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
[#] Добавить update и/или draw в objects
[#] Рисование 2д объектов
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
}

impl Render {
    pub fn new(mut app: App, modules: Modules) -> Self {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();


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


        app.objects2d.update();


        Self {
            app,
            modules,
            ctx,
            pipeline,
        }
    }
}

impl EventHandler for Render {
    fn update(&mut self) {
        self.app = self.modules.update(&mut self.app);
        self.app.objects2d.update();
    }

    fn draw(&mut self) {
        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.pipeline);


        let canvas = vec2(50., 50.);

        let window = window::screen_size();
        let window = vec2(window.0, window.1) / 2.;

        let aspect_window = window.x / window.y;
        let aspect_canvas = canvas.x / canvas.y;

        let scale = canvas.x / (aspect_canvas / aspect_window);
        let mvp = Mat4::orthographic_rh_gl(
            -scale,
            scale,
            -canvas.y,
            canvas.y,
            -1.,
            1.
        );


        for draw in self.app.objects2d.get_draw() {
            self.ctx.apply_uniforms(UniformsSource::table(&shader::Uniforms {
                mvp,
                transform: draw.2,
            }));

            let vertex_buffer = self.ctx.new_buffer(
                BufferType::VertexBuffer,
                BufferUsage::Immutable,
                BufferSource::slice(draw.0),
            );

            let index_buffer = self.ctx.new_buffer(
                BufferType::IndexBuffer,
                BufferUsage::Immutable,
                BufferSource::slice(draw.1),
            );

            let bindings = Bindings {
                vertex_buffers: vec![vertex_buffer],
                index_buffer: index_buffer,
                images: vec![],
            };

            self.ctx.apply_bindings(&bindings);
            self.ctx.draw(0, draw.1.len() as i32, 1);
        }

        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}