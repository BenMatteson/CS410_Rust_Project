extern crate gfx_device_gl;

use entity::{Entity, Direction};
use graphics::Transformed;
use opengl_graphics::{GlGraphics, Texture};
use piston::input::RenderArgs;
use piston::input::*;
use piston_window::{rectangle, DrawState, Image};

const SIZE: f64 = 0.03;
const SPEED: f64 = 500.0;
const START: (f64, f64) = (0.0, 0.0);

pub struct Player {
    pos: (f64, f64),
    texture: Texture,
    movement: Direction,
    size: f64,
}

impl Player {
    pub fn new(texture: Texture) -> Player {
        Player {
            pos: START,
            texture,
            size: SIZE,
            movement: Direction {
                up: 0.0,
                down: 0.0,
                left: 0.0,
                right: 0.0,
            },
        }
    }

    pub fn movement(&mut self, key: Key, release: bool) {
        match key {
            Key::W => match release {
                false => self.movement.up = SPEED,
                true => self.movement.up = 0.0,
            },
            Key::A => match release {
                false => self.movement.left = SPEED,
                true => self.movement.left = 0.0,
            },
            Key::S => match release {
                false => self.movement.down = SPEED,
                true => self.movement.down = 0.0,
            },
            Key::D => match release {
                false => self.movement.right = SPEED,
                true => self.movement.right = 0.0,
            },
            _ => (),
        };
    }
}

impl Entity for Player {
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
