extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use std::time::Duration;

fn main() {
    // Initialisation SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Création de la fenêtre
    let window = video_subsystem
        .window("AgeOfRust", 1600, 1200)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Boucle principale du jeu
    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main_loop,
                _ => {}
            }
        }

        // Gestion des entrées du joueur
        let keys: Vec<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        if keys.contains(&Keycode::Left) {
            println!("Toto");
        }
        if keys.contains(&Keycode::Right) {
            println!("Tata");
        }

        // Efface l'écran
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Met à jour l'écran
        canvas.present();

        // Limite la vitesse de rafraîchissement
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
