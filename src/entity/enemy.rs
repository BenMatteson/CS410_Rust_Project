use entity::projectile::Projectile;
use entity::*;

use opengl_graphics::Texture;
use piston::input::UpdateArgs;
use rand::{thread_rng, Rng};

const SPEED: f64 = 80.0;

const SHOT_SPEED: f64 = 150.0;
const SHOT_DAMAGE: i64 = 10;

const LOW_BOUND: f64 = -20.0;
const HIGH_BOUND: f64 = 500.0;

// staticallly store textures
lazy_static! {
    static ref TEXTURE: Texture = load_texture("enemy.png");
}
lazy_static! {
    static ref SHOT_TEXTURE: Texture = load_texture("enemy_shot.png");
}

/// An enemy Entity
pub struct Enemy {
    pos: (f64, f64),
    size: f64,
    movement: Movement,
    alive: bool,
    hp: i64,
    shot_timer: usize,
    score: i64,
}

impl Enemy {
    pub fn new(size: f64, pos: (f64, f64), hp: i64) -> Enemy {
        let mut drift_down = Movement::new();
        drift_down.down = SPEED;
        let mut rng = thread_rng();
        Enemy {
            pos,
            size,
            movement: drift_down,
            alive: true,
            hp,
            shot_timer: rng.gen_range(10, 90), //initial delay
            score: -50,                        // player looses points for enemies that escape
        }
    }
}

impl Entity for Enemy {
    fn pos(&self) -> (f64, f64) {
        (self.pos.0, self.pos.1)
    }
    fn texture(&self) -> &Texture {
        &TEXTURE
    }
    fn size(&self) -> f64 {
        self.size
    }
    fn update(&mut self, args: &UpdateArgs) -> Option<Vec<Box<Entity>>> {
        self.pos = self.movement.applied_to(self.pos, args.dt);

        if self.pos.1 < LOW_BOUND || self.pos.1 > HIGH_BOUND {
            self.alive = false;
            return None;
        }

        if self.shot_timer == 0 {
            let mut rng = thread_rng();
            self.shot_timer = rng.gen_range(90, 240);
            let mut shot: Vec<Box<Entity>> = Vec::new();
            let mut shot_movement = Movement::new();
            shot_movement.down = SHOT_SPEED;
            shot.push(Box::new(Projectile::new(
                self.pos,
                shot_movement,
                Team::Player,
                SHOT_DAMAGE,
                10.0,
                &SHOT_TEXTURE,
            )));
            Some(shot)
        } else {
            self.shot_timer -= 1;
            None
        }
    }

    fn damage(&mut self, amount: i64) {
        self.hp -= amount;
        if self.hp <= 0 {
            self.score = 100;
            self.alive = false;
        }
    }

    fn alive(&self) -> bool {
        self.alive
    }

    fn team(&self) -> Team {
        Team::Enemy
    }

    fn score(&self) -> i64 {
        self.score
    }
}
