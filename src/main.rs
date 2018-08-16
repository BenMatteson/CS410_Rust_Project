// Loosely based on code from the Piston tutorials and examples
// https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started
// https://github.com/PistonDevelopers/piston-examples

//#![allow(dead_code)]
//#![allow(unused_variables)]
//#![allow(unused_imports)]

extern crate find_folder;
extern crate graphics;
extern crate odds;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;
#[macro_use]
extern crate lazy_static;

use odds::vec::VecExt;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston_window::{PistonWindow, TextureSettings, WindowSettings};
use rand::{thread_rng, Rng};

mod entity;

use entity::enemy::Enemy;
use entity::player::Player;
use entity::*;

const ENEMY_HEALTH: i64 = 30;

const TOP_TEXT_HEIGHT: f64 = 20.0;
const BOTTOM_TEXT_HEIGHT: f64 = 480.0;
const SCORE_X_POS: f64 = 300.0;

/// stores the game state
pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    player: Player,
    entities: Vec<Box<Entity>>,
    spawn_timer: usize,
    score: i64,
    high_score: i64,
}

impl App {
    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache) {
        use graphics::*;
        // renders all game objects. objects lower in the function will be rendered on top

        const GREEN: [f32; 4] = [0.0, 0.4, 0.25, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        // Clear the screen.
        clear(GREEN, &mut self.gl);

        // Draw enemies and projectiles
        for entity in self.entities.iter_mut() {
            entity.render(&mut self.gl, args);
        }

        //Draw the player
        self.player.render(&mut self.gl, args);

        // Draw the UI
        let health = format!("Health: {}", self.player.get_health());
        let score = format!("Score: {}", self.score);
        let high_score = format!("High Score: {}", self.high_score);
        self.gl.draw(args.viewport(), |c, gl| {
            text::Text::new_color(WHITE, 20)
                .draw(
                    &health,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(0.0, TOP_TEXT_HEIGHT),
                    gl,
                )
                .unwrap();
            text::Text::new_color(WHITE, 20)
                .draw(
                    &score,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(SCORE_X_POS, TOP_TEXT_HEIGHT),
                    gl,
                )
                .unwrap();
            text::Text::new_color(WHITE, 20)
                .draw(
                    &high_score,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(0.0, BOTTOM_TEXT_HEIGHT),
                    gl,
                )
                .unwrap();
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // collisions block
        {
            let mut collisions = Vec::new();
            let mut player_collisions = Vec::new();
            // compare each element to find collisions, can skip preceding since they were already compared
            for entity in self.entities.iter().enumerate() {
                // compare to remaining items on the list, skip elements up to and including self (index + 1)
                for other in self.entities.iter().enumerate().skip(entity.0 + 1) {
                    if check_collision(entity.1.as_ref(), other.1.as_ref()) {
                        // store collisions as pairs of indexes
                        collisions.push((entity.0, other.0))
                    }
                }
                // also check for collisions with the player
                if check_collision(&self.player, entity.1.as_ref()) {
                    player_collisions.push(entity.0);
                }
            }
            for (entity1_index, entity2_index) in collisions {
                // split the vec to be able to mutate both elements, split after element1 so it keeps its index
                // element2 becomes the first element of the second slice.
                let (entity1_slice, entity2_slice) = self.entities.split_at_mut(entity2_index);
                let entity1 = entity1_slice[entity1_index].as_mut();
                let entity2 = entity2_slice[0].as_mut();
                collide(entity1, entity2);
            }
            for entity_index in player_collisions {
                let entity = self.entities[entity_index].as_mut();
                collide(&mut self.player, entity);
            }
        }

        // Spawn new enemies
        if self.spawn_timer == 0 {
            let mut rng = thread_rng();
            let x_pos = rng.gen_range(20_f64, 620_f64);
            let enemy_size = 20.0;
            let new_enemy = Enemy::new(enemy_size, (x_pos, -10.0), ENEMY_HEALTH);
            self.entities.push(Box::new(new_enemy));
            self.spawn_timer = rng.gen_range(60, 240);
        } else {
            self.spawn_timer -= 1;
        }

        // update player
        match self.player.update(args) {
            Some(mut shots) => self.entities.append(&mut shots),
            None => (),
        }
        if self.player.get_health() <= 0 {
            self.score = 0;
            self.player.reset();
        }

        // update all other entities,
        let mut score = 0;
        let mut new_entities = Vec::new();
        self.entities.retain_mut(|entity| {
            match entity.update(args) {
                Some(mut shots) => new_entities.append(&mut shots),
                None => {}
            }
            if entity.alive() {
                true
            } else {
                score += entity.score();
                false
            }
        });
        self.entities.append(&mut new_entities);

        self.score += score;
        self.high_score = i64::max(self.high_score, self.score);
    }

    fn key(&mut self, args: &ButtonArgs) {
        let release = match args.state {
            ButtonState::Press => false,
            ButtonState::Release => true,
        };

        match args.button {
            Button::Keyboard(key @ Key::W) => self.player.movement(key, release),
            Button::Keyboard(key @ Key::A) => self.player.movement(key, release),
            Button::Keyboard(key @ Key::S) => self.player.movement(key, release),
            Button::Keyboard(key @ Key::D) => self.player.movement(key, release),
            Button::Keyboard(Key::Space) => self.player.set_firing(!release),
            _ => (),
        }
    }
}

fn collide(entity1: &mut Entity, entity2: &mut Entity) {
    //eprintln!("collision");
    entity1.collide(entity2);
    entity2.collide(entity1);
}

fn check_collision(entity1: &Entity, entity2: &Entity) -> bool {
    let (x1, y1) = entity1.pos();
    let (x2, y2) = entity2.pos();
    let dx = x1 - x2;
    let dy = y1 - y2;
    let dist = { (dx * dx) + (dy * dy) }.sqrt();
    dist < entity1.size() + entity2.size()
}

#[test]
fn test_check_collision() {
    struct CollisionTestEntity {
        pos: (f64, f64),
        size: f64,
    }
    impl Entity for CollisionTestEntity {
        fn pos(&self) -> (f64, f64) {
            self.pos
        }

        fn texture(&self) -> &opengl_graphics::Texture {
            unimplemented!()
        }

        fn size(&self) -> f64 {
            self.size
        }

        fn team(&self) -> Team {
            unimplemented!()
        }
    }
    let entity1 = &CollisionTestEntity {
        pos: (0.0, 0.0),
        size: 5.0,
    };
    let entity2 = &CollisionTestEntity {
        pos: (0.0, 10.0),
        size: 5.0,
    };
    let entity3 = &CollisionTestEntity {
        pos: (10.0, 0.0),
        size: 5.001,
    };
    assert_eq!(check_collision(entity1, entity2), false);
    assert_eq!(check_collision(entity1, entity3), true);
    assert_eq!(check_collision(entity2, entity3), false);
}

fn main() {
    let opengl = OpenGL::V3_2;
    // Create a pistonwindow.
    let mut window: PistonWindow = WindowSettings::new("Game", [640, 480])
        .opengl(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let font = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap()
        .join("FiraSans-Regular.ttf");
    let mut glyphs: GlyphCache = GlyphCache::new(font, (), TextureSettings::new()).unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        player: Player::new(),
        entities: Vec::new(),
        spawn_timer: 0,
        score: 0,
        high_score: 0,
    };

    // core game loop, default ticks/sec = 120
    // apparently this is very inconsistent though, after running on another machine I found it was
    // nearly half as fast, not sure what to do about it at this point...
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r, &mut glyphs);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(k) = e.button_args() {
            app.key(&k);
        }
    }
}
