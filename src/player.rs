//mod personnages;
pub struct Player {
    pub life_points: i32,
    pub wallet: i32,
    //direction edge,
}

impl Player {
    pub fn new() -> Self {
        Player {
            life_points: 1000,
            wallet: 1000,
        }
    }

    pub fn decrease_life(&mut self, amount: i32) {
        self.life_points -= amount;
        if self.life_points < 0 {
            self.life_points = 0;
            //TODO: notify death
        }
    }

    fn increase_money(&mut self, amount: i32) {
        self.wallet += amount;
        if self.wallet < 0 {
            self.wallet = 0;
        }
    }

    fn decrease_money(&mut self, amount: i32) -> bool {
        if self.wallet > 0 {
            self.wallet -= amount;
            true
        }
        else {
            false
        }
    }
}
