extern crate gfx_device_gl;

use entity::Entity;
use graphics::{Image, Transformed};
use opengl_graphics::{GlGraphics, Texture};
use piston::input::RenderArgs;
use piston::input::*;
use piston_window::{rectangle, DrawState};

const SPEED: f64 = 500.0;
const SIZE: f64 = 0.03;
const START: (f64, f64) = (0.0, 0.0);

pub struct Enemy {
    pos: (f64, f64),
    texture: Texture,
    size: f64,
}

impl Enemy {
    pub fn new(texture: Texture, size: f64) -> Enemy {
        Enemy {
            pos: (0.0, 0.0),
            texture,
            size,
        }
    }
}

impl Entity for Enemy {
    fn pos(&self) -> (f64, f64) {
        (self.pos.0, self.pos.1)
    }
    fn texture(&self) -> &Texture {
        &self.texture
    }
    fn size(&self) -> f64 {
        self.size
    }
    fn update(&mut self, args: &UpdateArgs) {}
}
