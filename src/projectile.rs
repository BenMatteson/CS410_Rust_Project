use entity::{Entity, Team, Direction, load_asset};
use opengl_graphics::Texture;
use piston::input::*;

pub struct Projectile {
    pos: (f64, f64),
    targets: Team,
    texture: Texture,
    size: f64,
    movement: Direction,
}

impl Projectile {
    pub fn new(targets: Team, pos: (f64, f64), movement: Direction) -> Projectile {
        Projectile {
            pos,
            targets,
            texture: load_asset("projectile.png"),
            size: 0.01,
            movement,
        }
    }
}

impl Entity for Projectile {
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