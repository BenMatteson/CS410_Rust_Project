use find_folder;
use graphics::Transformed;
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use piston::input::RenderArgs;
use piston::input::*;
use piston_window::{rectangle, DrawState, Image};
//use rand::{thread_rng, Rng};

pub trait Entity {
    //    fn id(&self) -> usize;
    fn pos(&self) -> (f64, f64);
    fn texture(&self) -> &Texture;
    fn size(&self) -> f64;
    fn update(&mut self, args: &UpdateArgs);
    fn collide(&mut self, other: &Entity) {}
    fn alive(&self) -> bool {
        true
    }
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

pub struct Direction {
    pub up: f64,
    pub down: f64,
    pub left: f64,
    pub right: f64,
}

impl Direction {
    pub fn new() -> Direction {
        Direction {
            up: 0.0,
            down: 0.0,
            left: 0.0,
            right: 0.0,
        }
    }
}

pub trait Player {}
pub trait Enemy {}

#[derive(PartialEq, Eq)]
pub enum Team {
    Player,
    Enemy,
}
