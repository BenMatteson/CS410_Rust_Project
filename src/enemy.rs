use entity::*;
use opengl_graphics::Texture;
use piston::input::*;

const SPEED: f64 = 500.0;
const SIZE: f64 = 0.03;
const START: (f64, f64) = (0.0, 0.0);

pub struct Enemy {
    //    id: usize,
    pos: (f64, f64),
    texture: Texture,
    size: f64,
    movement: Direction,
}

impl Enemy {
    pub fn new(texture: Texture, size: f64, pos: (f64, f64)) -> Enemy {
        let mut drift_down = Direction::new();
        drift_down.down = 50.0;
        Enemy {
            //            id: rand_id(),
            pos,
            texture,
            size,
            movement: drift_down,
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
        let (x, y) = self.pos;
        let x_movement = self.movement.right - self.movement.left;
        let y_movement = self.movement.down - self.movement.up;
        self.pos = (x + (x_movement * args.dt), y + (y_movement * args.dt))
    }
}
