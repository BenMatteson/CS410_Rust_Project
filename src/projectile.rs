use entity::{Entity, Team, Direction, load_asset};
use opengl_graphics::Texture;
use piston::input::*;

// bounds for automatically cleaning up stray projectiles
const LOW_BOUND: f64 = -10.0;
const HIGH_BOUND: f64 = 490.0;

pub struct Projectile {
    pos: (f64, f64),
    targets: Team,
    texture: Texture,
    size: f64,
    movement: Direction,
    alive: bool,
}

impl Projectile {
    pub fn new(targets: Team, pos: (f64, f64), movement: Direction) -> Projectile {
        Projectile {
            pos,
            targets,
            texture: load_asset("projectile.png"),
            size: 0.01,
            movement,
            alive: true,
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
    fn alive(&self) -> bool {
        self.alive
    }
    fn update(&mut self, args: &UpdateArgs) {
        let (x, y) = self.pos;
        let x_movement = self.movement.right - self.movement.left;
        let y_movement = self.movement.down - self.movement.up;
        self.pos = (x + (x_movement * args.dt), y + (y_movement * args.dt));

        if self.pos.1 < LOW_BOUND || self.pos.1 > HIGH_BOUND {
            self.alive = false;
        }
    }
}