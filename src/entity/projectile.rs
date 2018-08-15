use entity::*;
use opengl_graphics::Texture;
use piston::input::UpdateArgs;

// bounds for automatically cleaning up stray projectiles
const LOW_BOUND: f64 = -10.0;
const HIGH_BOUND: f64 = 490.0;

pub struct Projectile {
    //    id: usize,
    pos: (f64, f64),
    targets: Team,
    texture: &'static Texture,
    size: f64,
    movement: Movement,
    alive: bool,
    damage: i64,
}

impl Projectile {
    pub fn new(
        pos: (f64, f64),
        movement: Movement,
        targets: Team,
        damage: i64,
        size: f64,
        texture: &'static Texture,
    ) -> Projectile {
        Projectile {
            //            id: rand_id(),
            pos,
            targets,
            texture,
            size,
            movement,
            alive: true,
            damage,
        }
    }
}

impl Entity for Projectile {
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
        let (x, y) = self.pos;
        let x_movement = self.movement.right - self.movement.left;
        let y_movement = self.movement.down - self.movement.up;
        self.pos = (x + (x_movement * args.dt), y + (y_movement * args.dt));

        if self.pos.1 < LOW_BOUND || self.pos.1 > HIGH_BOUND {
            self.alive = false;
        }
        None
    }
    fn collide(&mut self, other: &mut Entity) {
        if other.team() == self.targets {
            if other.damage(self.damage) {
                self.alive = false;

            }
        }
    }
    fn alive(&self) -> bool {
        self.alive
    }
    fn team(&self) -> Team {
        Team::Immune
    }
}
