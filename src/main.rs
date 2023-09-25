use macroquad::prelude::*;

mod personnage;

use personnage::MovingRect;

async fn main() {
    let mut rectangles = Vec::new();

    // Ajoutez autant de rectangles que nécessaire dans le vecteur
    rectangles.push(MovingRect::new(0.0, 100.0, 50.0, 50.0, 2.0));
    rectangles.push(MovingRect::new(0.0, 200.0, 50.0, 50.0, 3.0));
    rectangles.push(MovingRect::new(0.0, 300.0, 50.0, 50.0, 1.5));

    loop {
        clear_background(WHITE);

        // Déplacez tous les rectangles dans le vecteur
        for rect in &mut rectangles {
            rect.move_right();
        }

        // Récupérez et dessinez tous les rectangles
        for rect in &rectangles {
            let (x, y) = rect.get_coordinates();
            draw_rectangle(x, y, rect.width, rect.height, RED);
        }

        next_frame().await;
    }
}
