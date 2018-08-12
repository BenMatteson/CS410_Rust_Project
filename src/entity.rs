use find_folder;
use graphics::Transformed;
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use piston::input::RenderArgs;
use piston::input::*;
use piston_window::{rectangle, DrawState, Image};
//use rand::{thread_rng, Rng};

const BASE_COLLISION_DAMAGE: i64 = 5;

pub trait Entity {
    //    fn id(&self) -> usize;
    fn pos(&self) -> (f64, f64);
    fn texture(&self) -> &Texture;
    fn size(&self) -> f64;
    #[allow(unused_variables)]
    fn update(&mut self, args: &UpdateArgs) -> Option<Vec<Box<Entity>>> {
        None
    }
    fn collide(&mut self, other: &mut Entity) {
        if self.team() != other.team() {
            other.damage(BASE_COLLISION_DAMAGE);
        }
    }
    #[allow(unused_variables)]
    fn damage(&mut self, amount: i64) -> bool {
        false
    }
    fn alive(&self) -> bool {
        true
    }
    fn team(&self) -> Team;
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let image = Image::new().rect(rectangle::centered(rectangle::square(
            0.0,
            0.0,
            //args.draw_width as f64 * self.size(),
            self.size(),
        )));
        let (x, y) = self.pos();
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y);
            image.draw(self.texture(), &DrawState::new_alpha(), transform, gl)
        });
    }
}

//impl PartialEq for Entity {
//    fn eq(&self, other: &Entity) -> bool {
//        self.id() == other.id()
//    }
//}

// generates random number to be used as id, no checking, not ideal
//pub fn rand_id() -> usize {
//    let mut rng = thread_rng();
//    rng.gen_range(usize::min_value(), usize::max_value())
//}

pub fn load_asset(asset: &str) -> Texture {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let file = assets.join(asset);

    let mut settings = TextureSettings::new();
    settings.set_generate_mipmap(false);
    settings.set_compress(false);

    Texture::from_path(&file, &settings).unwrap()
}

pub struct Movement {
    pub up: f64,
    pub down: f64,
    pub left: f64,
    pub right: f64,
}

impl Movement {
    pub fn new() -> Movement {
        Movement {
            up: 0.0,
            down: 0.0,
            left: 0.0,
            right: 0.0,
        }
    }

    pub fn applied_to(&self, pos: (f64, f64), delta_time: f64) -> (f64, f64) {
        let (mut x, mut y) = pos;
        let x_movement = (self.right - self.left) * delta_time;
        let y_movement = (self.down - self.up) * delta_time;
        x += x_movement;
        y += y_movement;
        (x, y)
    }
}

#[derive(PartialEq, Eq)]
pub enum Team {
    Player,
    Enemy,
    Immune,
}
