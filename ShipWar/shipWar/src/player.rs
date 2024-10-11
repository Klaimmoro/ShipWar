use crate::ship::Ship;
use crate::cell::Cell;
use std::io;
//СДЕЛАТЬ ВВДИМОЕ КОЛ-ВО КОРАБЛЕЙ
pub struct Player {
    pub  player_ships: Vec<Ship>,
    pub game_field: Vec<Cell>,
    pub turn: bool,
    pub player_num: u8
} 

impl Player{
    pub fn new(count_ship: u8, num: u8) -> Self{
        let mut tmp_ships: Vec<Ship> = Vec::new();
        let mut tmp_field: Vec<Cell> = Vec::new();
        for i in 0..count_ship {
            println!("Player {} ",num);
            println!("Enter coordinates to {} ship",i);
            let mut x_str = String::new();
            let mut y_str = String::new();

            match io::stdin().read_line(&mut x_str) {
                Ok(_) => {},
                Err(e) => println!("Input error! {}",e)
            }

            match io::stdin().read_line(&mut y_str) {
                Ok(_) => {},
                Err(e) => println!("Input error! {}",e)
            }

            let x: u8 = x_str.trim().parse().unwrap();
            let y: u8 = y_str.trim().parse().unwrap();
        
            tmp_ships.push(Ship::new(x,y));

        }   

        for i in 0..5{
            for j in 0..5{
                tmp_field.push(Cell::new(i,j));
            }
        }


        Self{player_ships: tmp_ships, turn: true, player_num: num, game_field: tmp_field  }
    }

    pub fn attack(&mut self) -> (u8,u8) {
        println!("Enter coordinates to ship");
        let mut x_str = String::new();
        let mut y_str = String::new();
        match io::stdin().read_line(&mut x_str) {
            Ok(_) => {},
            Err(e) => println!("Input error! {}",e)
        }
    
        match io::stdin().read_line(&mut y_str) {
            Ok(_) => {},
            Err(e) => println!("Input error! {}",e)
        }

        let x: u8 = x_str.trim().parse().unwrap();
        let y: u8 = y_str.trim().parse().unwrap();

        self.turn = false;

        (x, y)

    }

    pub fn defense(&mut self, x: u8, y:u8)-> u8{
        let mut _counter: usize = 0;
        let mut miss: u8 = 0;
        for _ships in &self.player_ships{
            if _ships.position.coords.x==x {
                if _ships.position.coords.x==y {
                    println!("Ship dead!");
                    miss = 1;
                    break;
                }
            }
            _counter+=1;
        }


        if miss==1 { self.player_ships.remove(_counter); }

        self.turn = true;

        miss

    }

    pub fn change_field(&mut self, case: u8, x: u8, y:u8){
        for cell in &mut self.game_field{
            if(cell.coords.x == x && cell.coords.y == y){
                cell.in_active(case);
            }
        }
    }

    pub fn draw_field(&self){
        let mut counter: u8 =0;
        for cell in &self.game_field{
            match cell.active{
                0 => print!("*"),
                1 => print!("✓"),
                2 => print!("X"),
                _ => print!("")
            }
            counter+=1;
            if counter%5==0 { println!("")}
        }
    }




}