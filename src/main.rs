use macroquad::prelude::*;

mod personnage;

use personnage::MovingRect;
use personnage::cote::Gauche;
use personnage::cote::Droit;

#[macroquad::main("Background Image Example", window_conf)]
async fn main() {
    let mut rectangles = Vec::new();

    // Ajoutez autant de rectangles que nécessaire dans le vecteur
    rectangles.push(MovingRect::new(Droit,800.0, 100.0, 50.0, 50.0, 2.0));
    rectangles.push(MovingRect::new(Droit,200.0, 200.0, 50.0, 50.0, 3.0));
    rectangles.push(MovingRect::new(Gauche,0.0, 100.0, 50.0, 50.0, 1.5));

    loop {
        clear_background(WHITE);

        // Déplacez tous les rectangles dans le vecteur
        for rect in &mut rectangles {
            rect.bouge();
        }

        // Récupérez et dessinez tous les rectangles
        for rect in &rectangles {
            let (x, y) = rect.get_coordinates();
            draw_rectangle(x, y, rect.width, rect.height, RED);

        }

        for n in 1..=3 {

        }

        next_frame().await;
    }
}
