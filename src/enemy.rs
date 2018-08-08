use entity::{Entity, Direction};
use opengl_graphics::Texture;
use piston::input::*;

const SPEED: f64 = 500.0;
const SIZE: f64 = 0.03;
const START: (f64, f64) = (0.0, 0.0);

pub struct Enemy {
    pos: (f64, f64),
    texture: Texture,
    size: f64,
    movement: Direction,
}

impl Enemy {
    pub fn new(texture: Texture, size: f64) -> Enemy {
        Enemy {
            pos: (0.0, 0.0),
            texture,
            size,
            movement: Direction::new(),
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
    fn update(&mut self, args: &UpdateArgs) {
        let (x, y) = self.pos;
        let x_movement = self.movement.right - self.movement.left;
        let y_movement = self.movement.down - self.movement.up;
        self.pos = (x + (x_movement * args.dt), y + (y_movement * args.dt))
    }
}
