#[macro_use] extern crate glium;
extern crate xml;
extern crate cage;
extern crate patchgllib;

use patchgllib::model::Patchwork;
use patchgllib::renderer::PatchRenderer;

fn main() {
    let xml = include_str!("screen_with_patch.xml");
    let patchwork = Patchwork::from_xml(xml);
    let patch = patchwork.patch;
    let patch_renderer = PatchRenderer::new(patch);
    loop {
        let mut target = patch_renderer.display.draw();
        use glium::{Surface};
        target.clear_color(0.70, 0.80, 0.90, 1.0);

        let uniforms = uniform! {
            modelview: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ]
        };

        target.draw(&patch_renderer.vertex_buffer,
                    &patch_renderer.indices,
                    &patch_renderer.program,
                    &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in patch_renderer.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 2 + 2);
    }

    #[test]
    fn patch_renders() {}
}
