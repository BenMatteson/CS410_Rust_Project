use entity::*;
use opengl_graphics::Texture;
use piston::input::*;

// bounds for automatically cleaning up stray projectiles
const LOW_BOUND: f64 = -10.0;
const HIGH_BOUND: f64 = 490.0;

pub struct Projectile {
    //    id: usize,
    pos: (f64, f64),
    targets: Team,
    texture: Texture,
    size: f64,
    movement: Direction,
    alive: bool,
    damage: u64,
}

impl Projectile {
    pub fn new(
        pos: (f64, f64),
        movement: Direction,
        targets: Team,
        damage: u64,
        size: f64,
    ) -> Projectile {
        Projectile {
            //            id: rand_id(),
            pos,
            targets,
            texture: load_asset("projectile.png"),
            size,
            movement,
            alive: true,
            damage,
        }
    }

    pub fn damage(&self) -> u64 {
        return self.damage;
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
    fn update(&mut self, args: &UpdateArgs) {
        let (x, y) = self.pos;
        let x_movement = self.movement.right - self.movement.left;
        let y_movement = self.movement.down - self.movement.up;
        self.pos = (x + (x_movement * args.dt), y + (y_movement * args.dt));

        if self.pos.1 < LOW_BOUND || self.pos.1 > HIGH_BOUND {
            self.alive = false;
        }
    }
    fn collide(&mut self, other: &Entity) {
        if other.team() == self.targets {
            // we have a hit!
            self.alive = false;
        }
    }
    fn alive(&self) -> bool {
        self.alive
    }
    fn team(&self) -> Team {
        Team::Projectile
    }
}
