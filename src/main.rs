extern crate sdl2;
extern crate sdl2_image;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};
use sdl2::image::{self, LoadTexture};
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = image::init(image::InitFlag::PNG | image::InitFlag::JPG).unwrap();

    let window = video_subsystem
        .window("SDL2 Image Example", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // Chargez votre image en tant que texture
    let image_texture = texture_creator.load_texture("chemin/vers/votre/image.png").unwrap();

    'main_loop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => break 'main_loop,
                _ => {}
            }
        }

        // Effacez l'écran avec une couleur de fond (optionnel)
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Dessinez l'image en tant que fond (en plein écran)
        canvas.copy(&image_texture, None, Some(Rect::new(0, 0, 800, 600))).unwrap();

        // Mise à jour de l'affichage
        canvas.present();

        // ... gestion du clavier ou autres actions ici ...
    }
}
