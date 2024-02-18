mod app;

use app::App;
use graphics::Graphics;
use opengl_graphics::{ OpenGL, GlGraphics };
use piston::{
    WindowSettings,
    RenderArgs,
    UpdateArgs,
    Events,
    EventSettings,
    RenderEvent,
    UpdateEvent,
};
use piston_window::PistonWindow;

fn main() {
    let opengl = OpenGL::V4_5;
    let mut window: PistonWindow = WindowSettings::new("title", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let gl = GlGraphics::new(opengl);

    let mut app = App { gl };
    run_loop(&mut app, &mut window);
}

fn run_loop(app: &mut App, w: &mut PistonWindow) {
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(w) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
