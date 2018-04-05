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
    surface_alpha: f32,
    shadow_color: [f32; 4],
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
        let surface_alpha = 1.0;
        let shadow_color = Color::custom_white(0.4).to_gl();
        let draw_parameters = glium::DrawParameters {
            depth: glium::Depth { test: glium::DepthTest::IfLess, write: true, ..Default::default() },
            blend: glium::Blend::alpha_blending(),
            smooth: Some(glium::Smooth::Nicest),
            ..Default::default()
        };
        ShadowRenderer { program, vertex_buffer, indices, modelview, surface_alpha, shadow_color, draw_parameters }
    }

    pub fn set_modelview(&mut self, modelview: [[f32; 4]; 4]) {
        self.modelview = modelview;
    }

    pub fn set_patch(&mut self, patch: &Patch, screen_dimensions: (f32, f32)) {
        self.vertex_buffer.write(&patch.shadow_trianglelist(screen_dimensions));
        self.surface_alpha = patch.color.a;
    }

    pub fn draw(&self, frame: &mut glium::Frame) {
        if self.surface_alpha > 0.0 {
            let uniforms = uniform! { modelview: self.modelview, uniformcolor: self.shadow_color };
            frame.draw(&self.vertex_buffer, &self.indices, &self.program, &uniforms, &self.draw_parameters).unwrap();
        }
    }
}

const PANEL_VERTEX_COUNT: usize = 6;
const PANEL_COUNT: usize = 3;
const SHADOW_TRIANGLELIST_VERTEX_COUNT: usize = PANEL_COUNT * PANEL_VERTEX_COUNT;

impl Patch {
    fn shadow_trianglelist(&self, screen_dim: (f32, f32)) -> Vec<Vertex> {
        let surface = self.cage.limits();
        let shadow = shadow_dim(screen_dim, surface);
        let (lt_bottom, rt_bottom, rb_bottom, lb_bottom) = bottom_panel(surface, shadow);
        let (lt_right, rt_right, rb_right, lb_right) = right_panel(surface, shadow);
        let (lt_left, rt_left, rb_left, lb_left) = left_panel(surface, shadow);
        vec![lt_bottom, rt_bottom, lb_bottom, lb_bottom, rt_bottom, rb_bottom,
             lt_right, rt_right, lb_right, lb_right, rt_right, rb_right,
             lt_left, rt_left, lb_left, lb_left, rt_left, rb_left,
        ]
    }
}

fn left_panel(surface: (f32, f32, f32, f32, f32, f32), shadow: (f32, f32, f32, f32)) -> (Vertex, Vertex, Vertex, Vertex) {
    let (surface_left, surface_right, surface_bottom, _surface_top, surface_far, _) = surface;
    let (shadow_left, shadow_top, _shadow_right, shadow_bottom) = shadow;

    let panel_top_x = if shadow_left > surface_left || surface_left == surface_right {
        shadow_left
    } else {
        surface_left
    };
    let lt_vertex = Vertex { position: [panel_top_x, shadow_top, surface_far] };
    let rt_vertex = Vertex { position: [panel_top_x, surface_bottom, surface_far] };
    let rb_vertex = Vertex { position: [shadow_left, shadow_bottom, 0.0] };
    let lb_vertex = Vertex { position: [shadow_left, shadow_top, 0.0] };
    (lt_vertex, rt_vertex, rb_vertex, lb_vertex)
}

fn right_panel(surface: (f32, f32, f32, f32, f32, f32), shadow: (f32, f32, f32, f32)) -> (Vertex, Vertex, Vertex, Vertex) {
    let (surface_left, surface_right, surface_bottom, _surface_top, surface_far, _) = surface;
    let (_shadow_left, shadow_top, shadow_right, shadow_bottom) = shadow;

    let panel_top_x = if shadow_right < surface_right || surface_left == surface_right {
        shadow_right
    } else {
        surface_right
    };
    let lt_vertex = Vertex { position: [panel_top_x, surface_bottom, surface_far] };
    let rt_vertex = Vertex { position: [panel_top_x, shadow_top, surface_far] };
    let rb_vertex = Vertex { position: [shadow_right, shadow_top, 0.0] };
    let lb_vertex = Vertex { position: [shadow_right, shadow_bottom, 0.0] };
    (lt_vertex, rt_vertex, rb_vertex, lb_vertex)
}

fn bottom_panel(surface: (f32, f32, f32, f32, f32, f32), shadow: (f32, f32, f32, f32)) -> (Vertex, Vertex, Vertex, Vertex) {
    let (surface_left, surface_right, surface_bottom, _surface_top, surface_far, _) = surface;
    let (shadow_left, _shadow_top, shadow_right, shadow_bottom) = shadow;

    let panel_top_left = if shadow_left > surface_left {
        shadow_left
    } else {
        surface_left
    };
    let panel_top_right = if shadow_right < surface_right {
        shadow_right
    } else {
        surface_right
    };
    let panel_top_y = if surface_bottom == _surface_top {
        shadow_bottom
    } else {
        surface_bottom
    };
    let lt_vertex = Vertex { position: [panel_top_left, panel_top_y, surface_far] };
    let rt_vertex = Vertex { position: [panel_top_right, panel_top_y, surface_far] };
    let rb_vertex = Vertex { position: [shadow_right, shadow_bottom, 0.0] };
    let lb_vertex = Vertex { position: [shadow_left, shadow_bottom, 0.0] };
    (lt_vertex, rt_vertex, rb_vertex, lb_vertex)
}

fn shadow_dim((screen_width, screen_height): (f32, f32),
              (left, right, bottom, top, far, _): (f32, f32, f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
    let screen_half_height = screen_height / 2.0;
    let light_x = screen_width / 2.0;
    let light_y = -0.25 * screen_half_height;
    let light_z = 3.0 * screen_half_height;
    let distance_top_from_light = top - light_y;
    let distance_bottom_from_light = bottom - light_y;
    let distance_left_from_light = left - light_x;
    let distance_right_from_light = right - light_x;

    let shadow_factor = light_z / (light_z - far);

    let shadow_top = light_y + distance_top_from_light * shadow_factor;
    let shadow_bottom = light_y + distance_bottom_from_light * shadow_factor;
    let shadow_left = light_x + distance_left_from_light * shadow_factor;
    let shadow_right = light_x + distance_right_from_light * shadow_factor;
    (shadow_left, shadow_top, shadow_right, shadow_bottom)
}
