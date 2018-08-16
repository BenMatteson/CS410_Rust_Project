use find_folder;
use graphics::Transformed;
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use piston::input::RenderArgs;
use piston::input::UpdateArgs;
use piston_window::{rectangle, DrawState, Image};

const BASE_COLLISION_DAMAGE: i64 = 5;

pub mod enemy;
pub mod player;
pub mod projectile;

pub trait Entity {
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
    fn damage(&mut self, amount: i64) {}
    fn alive(&self) -> bool {
        true
    }
    fn team(&self) -> Team;
    /// the effect the entity will have on the score when it dies
    fn score(&self) -> i64 {
        0
    }
    /// renders the texture returned by texture(), at the size returned by size()
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

/// utility function to load the desired texture from the 'assets' folder
pub fn load_texture(asset: &str) -> Texture {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let file = assets.join(asset);

    let mut settings = TextureSettings::new();
    settings.set_generate_mipmap(false);
    settings.set_compress(false);

    Texture::from_path(&file, &settings).unwrap()
}

/// struct to track movement, allows competing forces to cancel each other out, allowing easier
/// managing of the player pressing keys for competing directions simultaneously
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

    /// applies the movement to an entity (or specifically it's position) using the given delta time
    /// for scale. returns the new position after the movement.
    // reads well, but a bit unintuitive.
    pub fn applied_to(&self, pos: (f64, f64), delta_time: f64) -> (f64, f64) {
        let (mut x, mut y) = pos;
        let x_movement = (self.right - self.left) * delta_time;
        let y_movement = (self.down - self.up) * delta_time;
        x += x_movement;
        y += y_movement;
        (x, y)
    }
}

/// the various 'teams' entities can have, used to ignore friendly fire, and keep projectiles from colliding
/// currently also describes the three structs that impl Entity, but this doesn't need to be true
#[derive(PartialEq, Eq)]
pub enum Team {
    Player,
    Enemy,
    Projectile,
}
