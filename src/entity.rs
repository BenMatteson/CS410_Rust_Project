use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use piston::input::*;

pub trait Entity {
    fn pos(&self) -> (f64, f64);
    fn update(&mut self, args: &UpdateArgs);
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs);
}
