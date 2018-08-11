use entity::*;
use opengl_graphics::Texture;
use piston::input::*;
use projectile::Projectile;

const SHOT_SPEED: f64 = 1000.0; // pixels/sec
const FIRE_RATE: f64 = 0.1; // sec/shot
const IFRAMES: usize = 30; // 120 ticks/sec
const BASE_HEALTH: i64 = 100;

const SIZE: f64 = 20.0; // pixels (radius?)
const SPEED: f64 = 500.0;
const START: (f64, f64) = (320.0, 420.0);

const MAX_X: f64 = 620.0;
const MAX_Y: f64 = 460.0;
const MIN_X: f64 = 20.0;
const MIN_Y: f64 = 20.0;

pub struct Player {
    //    id: usize,
    pos: (f64, f64),
    texture: Texture,
    movement: Direction,
    size: f64,
    shot_delay: f64,
    iframes: usize,
    health: i64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            //            id: rand_id(),
            pos: START,
            texture: load_asset("ship.png"),
            size: SIZE,
            movement: Direction::new(),
            shot_delay: 0.0,
            iframes: 0,
            health: BASE_HEALTH,
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
        let mut volley: Vec<Box<Entity>> = Vec::new();
        if self.shot_delay <= 0.0 {
            let mut direction = Direction::new();
            direction.up = SHOT_SPEED;
            // volley has only single projectile for now
            volley.push(Box::new(Projectile::new(
                self.pos,
                direction,
                Team::Enemy,
                10,
                SIZE / 3.0,
            )));
            self.shot_delay = FIRE_RATE;
        }
        volley
    }
}

impl Entity for Player {
    //    fn id(&self) -> usize {
    //        self.id
    //    }
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
        if self.pos.0 < MIN_X {
            self.pos.0 = MIN_X;
        }
        if self.pos.0 > MAX_X {
            self.pos.0 = MAX_X;
        }
        if self.pos.1 < MIN_Y {
            self.pos.1 = MIN_Y;
        }
        if self.pos.1 > MAX_Y {
            self.pos.1 = MAX_Y;
        }

        self.shot_delay -= args.dt;
        self.iframes = self.iframes.saturating_sub(1);
    }
    fn damage(&mut self, amount: i64) -> bool {
        if self.iframes == 0 {
            self.health -= amount;
            self.iframes = IFRAMES;
            true
        }
        else { false }
    }
    fn team(&self) -> Team {
        Team::Player
    }
}
