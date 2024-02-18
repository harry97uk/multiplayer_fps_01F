use graphics::Graphics;
use opengl_graphics::GlGraphics;
use piston::{ RenderArgs, UpdateArgs };

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct App {
    pub(crate) gl: GlGraphics,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, g| {
            g.clear_color(WHITE);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {}
}
