use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use piston::input::*;
use piston_window::{rectangle, DrawState, Image};
use graphics::Transformed;
use opengl_graphics::{Texture};

pub trait Entity {
    fn pos(&self) -> (f64, f64);
    fn texture(&self) -> &Texture;
    fn size(&self) -> f64;
    fn update(&mut self, args: &UpdateArgs);
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let image = Image::new().rect(rectangle::centered(rectangle::square(
            0.0,
            0.0,
            args.draw_width as f64 * self.size(),
        )));
        let (x, y) = self.pos();
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y);
            image.draw(self.texture(), &DrawState::new_alpha(), transform, gl)
        });
    }
}

pub struct Direction {
    pub up: f64,
    pub down: f64,
    pub left: f64,
    pub right: f64,
}