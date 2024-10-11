
use crate::player::Player;



pub struct Game{
    pub players: Vec<Player>

}

impl Game {
    pub fn new() -> Self{
        
        let mut tmp_players: Vec<Player> = Vec::new();


        for c in 0..2{
            tmp_players.push(Player::new(5,c));
        }

        Self{
            players: tmp_players
        }
    }


}