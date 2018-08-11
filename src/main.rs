// Based on code from the Piston tutorials and examples
// https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started
// https://github.com/PistonDevelopers/piston-examples

#![allow(dead_code)]
//#![allow(unused_variables)]
//#![allow(unused_imports)]

extern crate find_folder;
extern crate graphics;
extern crate odds;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;

use odds::vec::VecExt;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston_window::{PistonWindow, WindowSettings};

mod enemy;
mod entity;
mod player;
mod projectile;

use entity::*;
use player::Player;

// TODO edge detection
// player is bounded, projectiles and enemies bounded on y
// for now assuming fixed sized window...

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    player: Player,
    entities: Vec<Box<Entity>>,
    fire_key_down: bool,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 0.4, 0.25, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
        });
        for entity in self.entities.iter_mut() {
            entity.render(&mut self.gl, args);
        }
        self.player.render(&mut self.gl, args);

        // TODO player health, score, other UI?
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.player.update(args);
        if self.fire_key_down { // player manages cooldown, passes any new projectiles (entities)
            self.entities.append(&mut self.player.fire());
        }
        { // collisions
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
                // split the vec to be able to mutate both elements, split after element1 so it keeps index
                // element2 becomes the first element of the second slice.
                let (entity1_slice, entity2_slice) = self.entities.split_at_mut(entity2_index);
                let thing1 = entity1_slice[entity1_index].as_mut();
                let thing2 = entity2_slice[0].as_mut();
                collide(thing1, thing2);
            }
            for entity_index in player_collisions {
                let entity = self.entities[entity_index].as_mut();
                collide(&mut self.player, entity);
            }
        }

        //TODO spawn enemies

        self.entities.retain_mut(|entity| {
            entity.update(args);
            entity.alive()
        })
    }

    fn key(&mut self, args: &ButtonArgs) {
        let release;
        match args.state {
            ButtonState::Press => release = false,
            ButtonState::Release => release = true,
        }

        match args.button {
            Button::Keyboard(key @ Key::W) => self.player.movement(key, release),
            Button::Keyboard(key @ Key::A) => self.player.movement(key, release),
            Button::Keyboard(key @ Key::S) => self.player.movement(key, release),
            Button::Keyboard(key @ Key::D) => self.player.movement(key, release),
            Button::Keyboard(Key::Space) => self.fire_key_down = !release,
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

        fn update(&mut self, args: &UpdateArgs) {
            unimplemented!()
        }

        fn team(&self) -> Team {
            unimplemented!()
        }
    }
    let entity1 = &CollisionTestEntity{ pos: (0.0, 0.0), size: 5.0};
    let entity2 = &CollisionTestEntity{ pos: (0.0, 10.0), size: 5.0};
    let entity3 = &CollisionTestEntity{ pos: (10.0, 0.0), size: 5.001};
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

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        player: Player::new(),
        entities: Vec::new(),
        fire_key_down: false,
    };

    // core game loop, default ticks/sec = 120
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(k) = e.button_args() {
            app.key(&k);
        }
    }
}
