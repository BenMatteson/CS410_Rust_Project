use piston::input::RenderArgs;
use piston::input::*;
use piston_window::{rectangle, DrawState, Image};
use graphics::Transformed;
use opengl_graphics::{Texture};
use find_folder;
use opengl_graphics::{
    GlGraphics, OpenGL, Texture as GlTexture, TextureSettings as GlTextureSettings,
};


pub trait Entity {
    fn pos(&self) -> (f64, f64);
    fn texture(&self) -> &Texture;
    fn size(&self) -> f64;
    fn update(&mut self, args: &UpdateArgs);
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let image = Image::new().rect(rectangle::centered(rectangle::square(
            0.0,
            0.0,
            args.draw_width as f64 * self.size(),
        )));
        let (x, y) = self.pos();
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y);
            image.draw(self.texture(), &DrawState::new_alpha(), transform, gl)
        });
    }
}

pub fn load_asset(asset: &str) -> Texture {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let file = assets.join(asset);

    let mut settings = GlTextureSettings::new();
    settings.set_generate_mipmap(false);
    settings.set_compress(false);

    GlTexture::from_path(&file, &settings).unwrap()
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

pub trait Player{}
pub trait Enemy{}

pub enum Team {
    Player,
    Enemy,
}