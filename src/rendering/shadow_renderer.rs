use Color;
use glium;
use glium::backend::Facade;
use glium::Surface;
use super::model::{Patch, Vertex};

pub struct ShadowRenderer {
    pub program: glium::Program,
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
    pub indices: glium::index::NoIndices,
    modelview: [[f32; 4]; 4],
    color: [f32; 4],
    draw_parameters: glium::DrawParameters<'static>,
}

impl ShadowRenderer {
    pub fn new<F: Facade>(display: &F, modelview: [[f32; 4]; 4]) -> Self {
        let vertex_shader_src = include_str!("shaders/patch_vertex_shader.glsl");
        let fragment_shader_src = include_str!("shaders/patch_fragment_shader.glsl");
        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
        let vertex_buffer = glium::VertexBuffer::empty_dynamic(
            display,
            SHADOW_TRIANGLELIST_VERTEX_COUNT).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let color = Color::new(0.5, 1.0, 0.0, 0.0).to_gl();
        let draw_parameters = glium::DrawParameters {
            depth: glium::Depth { test: glium::DepthTest::IfLess, write: true, ..Default::default() },
            blend: glium::Blend::alpha_blending(),
            smooth: Some(glium::Smooth::Nicest),
            ..Default::default()
        };
        ShadowRenderer { program, vertex_buffer, indices, modelview, color, draw_parameters }
    }

    pub fn set_modelview(&mut self, modelview: [[f32; 4]; 4]) {
        self.modelview = modelview;
    }

    pub fn set_patch(&mut self, patch: &Patch, screen_dimensions: (f32, f32)) {
        self.vertex_buffer.write(&patch.shadow_trianglelist(screen_dimensions));
    }

    pub fn draw(&self, frame: &mut glium::Frame) {
        let uniforms = uniform! { modelview: self.modelview, uniformcolor: self.color };
        frame.draw(&self.vertex_buffer, &self.indices, &self.program, &uniforms, &self.draw_parameters).unwrap();
    }
}

const SHADOW_TRIANGLELIST_VERTEX_COUNT: usize = 6;

impl Patch {
    fn shadow_trianglelist(&self, (screen_width, screen_height): (f32, f32)) -> Vec<Vertex> {
        let screen_half_height = screen_height / 2.0;
        let light_x = screen_width / 2.0;
        let light_y = -2.0 * screen_half_height;
        let light_z = 3.0 * screen_half_height;

        let (left, right, bottom, _, far, _) = self.cage.limits();
        let shadow_factor = light_z / (light_z - far);
        let distance_bottom_from_light = bottom - light_y;
        let shadow_bottom = light_y + distance_bottom_from_light * shadow_factor;
        let distance_left_from_light = left - light_x;
        let shadow_left = light_x + distance_left_from_light * shadow_factor;
        let distance_right_from_light = right - light_x;
        let shadow_right = light_x + distance_right_from_light * shadow_factor;

        let panel_top = bottom;
        let panel_bottom = 0.0;
        let panel_bottom_left = shadow_left;
        let panel_bottom_right = shadow_right;
        let lt_vertex = Vertex { position: [left, panel_top, far] };
        let rt_vertex = Vertex { position: [right, panel_top, far] };
        let rb_vertex = Vertex { position: [panel_bottom_right, shadow_bottom, panel_bottom] };
        let lb_vertex = Vertex { position: [panel_bottom_left, shadow_bottom, panel_bottom] };
        vec![lt_vertex, rt_vertex, lb_vertex, lb_vertex, rt_vertex, rb_vertex]
    }
}
