use entity::*;
use projectile::Projectile;

use opengl_graphics::Texture;
use piston::input::*;
use rand::{thread_rng, Rng};

const SPEED: f64 = 80.0;

const SHOT_SPEED: f64 = 150.0;
const SHOT_DAMAGE: i64 = 10;

const LOW_BOUND: f64 = -20.0;
const HIGH_BOUND: f64 = 500.0;

pub struct Enemy {
    //    id: usize,
    pos: (f64, f64),
    texture: Texture,
    size: f64,
    movement: Movement,
    alive: bool,
    hp: i64,
    shot_timer: usize,
}

impl Enemy {
    pub fn new(size: f64, pos: (f64, f64), hp: i64) -> Enemy {
        let mut drift_down = Movement::new();
        drift_down.down = SPEED;
        let mut rng = thread_rng();
        Enemy {
            //            id: rand_id(),
            pos,
            texture: load_asset("enemy.png"),
            size,
            movement: drift_down,
            alive: true,
            hp,
            shot_timer: rng.gen_range(10, 90), //initial delay
        }
    }
}

impl Entity for Enemy {
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
            let texture = load_asset("enemy_shot.png");
            shot.push(Box::new(Projectile::new(self.pos, shot_movement, Team::Player, SHOT_DAMAGE, 10.0, texture)));
            Some(shot)
        } else {
            self.shot_timer -= 1;
            None
        }
    }

    fn damage(&mut self, amount: i64) -> bool {
        //println!("enemy damaged, {} -> {}", self.hp, self.hp - amount);
        self.hp -= amount;
        if self.hp <= 0 {
            self.alive = false;
            //println!("enemy died");
        }
        true
    }

    fn alive(&self) -> bool {
        self.alive
    }

    fn team(&self) -> Team {
        Team::Enemy
    }
}
