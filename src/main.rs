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

use entity::Entity;
use player::Player;

// TODO edge detection

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
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.player.update(args);
        if self.fire_key_down {
            self.entities.append(&mut self.player.fire());
        }
        let mut collisions = Vec::new();
        {
            for entity in self.entities.iter().enumerate() {
                let (x1, y1) = entity.1.pos();
                let entity_size = entity.1.size();
                let entity_index = entity.0;
                // compare to remaining items on the list, skip elements up to and including self (index + 1)
                for other in self.entities.iter().enumerate().skip(entity_index + 1) {
                    // calculate collision
                    let (x2, y2) = other.1.pos();
                    let dx = x1 - x2;
                    let dy = y1 - y2;
                    let dist = { (dx * dx) + (dy * dy) }.sqrt();
                    if dist < entity_size + other.1.size() {
                        // store collisions as pairs of indexes
                        collisions.push((entity_index, other.0))
                    }
                }
            }

            for (thing1, thing2) in collisions {
                let (thing1_side, thing2_side) = self.entities.split_at_mut(thing2);
                let thing1 = thing1_side[thing1].as_mut();
                let thing2 = thing2_side[0].as_mut();
                thing1.collide(thing2);
                thing2.collide(thing1);
            }
        }
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
