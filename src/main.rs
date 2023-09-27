mod player;

use player::Player;

fn main() {
    // Initialize player1 and player2
    let mut player1 = Player::new();
    let mut player2 = Player::new();

    // Decrease player1's life by 200 points
    player1.decrease_life(200);

    // Decrease player2's life by 150 points
    player2.decrease_life(150);

    // Print the remaining life points of both players
    println!("Player 1's remaining life: {}", player1.life_points);
    println!("Player 2's remaining life: {}", player2.life_points);
}
