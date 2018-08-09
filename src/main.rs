// Based on code from the Piston tutorials and examples
// https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started
// https://github.com/PistonDevelopers/piston-examples

#![allow(dead_code)]
//#![allow(unused_variables)]
//#![allow(unused_imports)]

extern crate find_folder;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate odds;

use piston::event_loop::*;
use piston::input::*;
use piston_window::{PistonWindow, WindowSettings};
use opengl_graphics::{GlGraphics, OpenGL};
use odds::vec::VecExt;


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
        // TODO calculate all collisions?
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
