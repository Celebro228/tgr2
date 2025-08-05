use miniquad::*;

use crate::app::App;
use crate::cross::*;
use crate::shader;

pub(crate) struct Render {
    ctx: Box<dyn RenderingBackend>,
    pipeline: Pipeline,
}

impl Render {
    pub(crate) fn new() -> Self {
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
                VertexAttribute::new("in_pos", VertexFormat::Float3),
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

        Self { ctx, pipeline }
    }

    pub(crate) fn draw(&mut self, app: &App) {
        self.ctx.begin_default_pass(Default::default());
        self.ctx.apply_pipeline(&self.pipeline);

        let canvas = vec2(50., 50.);
        let window = window::screen_size();
        let window = vec2(window.0, window.1) / 2.;
        let aspect_window = window.x / window.y;
        let aspect_canvas = canvas.x / canvas.y;
        let scale = canvas.x / (aspect_canvas / aspect_window);
        let mvp = Mat4::orthographic_rh_gl(-scale, scale, -canvas.y, canvas.y, -1., 1.);

        for draw in app.draw() {
            self.ctx
                .apply_uniforms(UniformsSource::table(&shader::Uniforms {
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
