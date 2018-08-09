use entity::{Entity, Direction, Team, load_asset};
use projectile::Projectile;
use opengl_graphics::Texture;
use piston::input::*;

const SIZE: f64 = 0.03;
const SPEED: f64 = 500.0;
const START: (f64, f64) = (0.0, 0.0);
const SHOT_SPEED: f64 = 1000.0;
const FIRE_RATE: f64 = 0.1;

pub struct Player {
    pos: (f64, f64),
    texture: Texture,
    movement: Direction,
    size: f64,
    shot_delay: f64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: START,
            texture: load_asset("ship.png"),
            size: SIZE,
            movement: Direction::new(),
            shot_delay: 0.0,
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

    pub fn fire(&mut self) -> Vec<Box<Entity>> {
        let mut volley:Vec<Box<Entity>> = Vec::new();
        if self.shot_delay <= 0.0 {
            let mut direction = Direction::new();
            direction.up = SHOT_SPEED;
            // volley has only single projectile for now
            volley.push(Box::new(Projectile::new(Team::Enemy, self.pos, direction)));
            self.shot_delay = FIRE_RATE;
        }
        volley
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
        self.pos = (x + (x_movement * args.dt), y + (y_movement * args.dt));

        self.shot_delay -= args.dt;
    }
}
