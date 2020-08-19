extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};


struct Game {
    gl:GlGraphics,
}

impl Game{
    fn render(&mut self, &RenderArgs) {
        use graphics;
        let RED: [f32;4] = [0.0, 1.0, 0.0, 1.0];    // Still this is a red background
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(RED, gl);
        })
    }   
} 

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Game",
        [400, 400]
    ).opengl(opengl).exit_on_esc(true).build().unwrap();

    let game = Game{
        gl:GlGraphics::new(opengl);
    }
    
    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
             game.render(&r);
        }
    }
}
