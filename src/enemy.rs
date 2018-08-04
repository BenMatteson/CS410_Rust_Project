extern crate gfx_device_gl;

use entity::Entity;
use graphics::{Image, Transformed};
use opengl_graphics::{GlGraphics, Texture};
use piston::input::RenderArgs;
use piston_window::{rectangle, DrawState};
use piston::input::*;

pub struct Enemy {
    pos: (f64, f64),
    texture: Texture,
}

impl Enemy {
    pub fn new(texture: Texture) -> Enemy {
        Enemy {
            pos: (0.0, 0.0),
            texture,
        }
    }
}

impl Entity for Enemy {
    fn pos(&self) -> (f64, f64) {
        (self.pos.0, self.pos.1)
    }
    fn update(&mut self, args: &UpdateArgs) {}
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let image = Image::new().rect(rectangle::square(0.0, 0.0, 40.0));
        let (x, y) = self.pos;
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y);

            image.draw(&self.texture, &DrawState::new_alpha(), transform, gl)
        });
    }
}
