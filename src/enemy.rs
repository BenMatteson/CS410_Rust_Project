use entity::*;
use opengl_graphics::Texture;
use piston::input::*;

const SPEED: f64 = 50.0;

const LOW_BOUND: f64 = -20.0;
const HIGH_BOUND: f64 = 500.0;

pub struct Enemy {
    //    id: usize,
    pos: (f64, f64),
    texture: Texture,
    size: f64,
    movement: Movement,
    alive: bool,
}

impl Enemy {
    pub fn new(texture: Texture, size: f64, pos: (f64, f64)) -> Enemy {
        let mut drift_down = Movement::new();
        drift_down.down = SPEED;
        Enemy {
            //            id: rand_id(),
            pos,
            texture,
            size,
            movement: drift_down,
            alive: true,
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
    fn update(&mut self, args: &UpdateArgs) {
        self.pos = self.movement.applied_to(self.pos, args.dt);

        if self.pos.1 < LOW_BOUND || self.pos.1 > HIGH_BOUND {
            self.alive = false;
            return;
        }

        // TODO fight back! enemies should shoot too
    }



    fn team(&self) -> Team {
        Team::Enemy
    }
}
