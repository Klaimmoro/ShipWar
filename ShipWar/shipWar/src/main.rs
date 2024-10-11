use std::io;
mod ship;
mod coordinate;
mod player;
mod game;
mod cell;
use cell::Cell;
use game::Game;
use ship::Ship;
use coordinate::Coordinate;



fn draw_menu(part: u8, player_num: u8){
    match part {
        1 => println!("Hello to shipWar game! \n a - new game \n b - exit"),
        2 => println!(" Player {} its your turn! \n a - attack \n b - exit", player_num),
        _ => println!(),
    }
}



fn main() {
    draw_menu(1,0);
    let mut select = String::new();
    match io::stdin().read_line(&mut select) {
        Ok(_) => {},
        Err(e) => println!("Input error! {}",e)
    }
    if select.trim() == "a"{
        select.clear();
        let mut _new_game: Game = Game::new();
        let mut breakGame: bool = false;
        // Ход игры
        while(!breakGame){
            select.clear();
            if _new_game.players[0].turn {

                draw_menu(2,_new_game.players[0].player_num);
                match io::stdin().read_line(&mut select) {
                    Ok(_) => {},
                    Err(e) => println!("Input error! {}",e)
                }
                
                if select.trim() == "a"{

                    //Атака
                    _new_game.players[0].draw_field();
                    let (x,y ) = _new_game.players[0].attack();
                    
                    let case: u8 = _new_game.players[1].defense(x, y);

                    _new_game.players[0].change_field(case, x, y);

                }

                else if select.trim() == "b"{
                    breakGame = true;
                    break;
                }

            }

            else if _new_game.players[1].turn{

                draw_menu(2,_new_game.players[1].player_num);
                match io::stdin().read_line(&mut select) {
                    Ok(_) => {},
                    Err(e) => println!("Input error! {}",e)
                }
                
                if select.trim() == "a"{

                    //Атака
                    _new_game.players[1].draw_field();

                    let (x,y ) = _new_game.players[1].attack();

                    let case: u8 = _new_game.players[0].defense(x, y);

                    _new_game.players[1].change_field(case, x, y);

                }
                else if select.trim() == "b"{
                    breakGame = true;
                    break;
                }

            }
        }


    }


    else if select.trim() == "b"{
        println!("Bye!");
        return;
    }



}
