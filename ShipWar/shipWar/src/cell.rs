use crate::coordinate::Coordinate;

pub struct Cell{
    pub coords: Coordinate,
    pub active: u8
}

impl Cell{
    pub fn new(x: u8,y:u8) -> Self{
       Self { coords: Coordinate::new(x, y), active: 0 }
    }

    pub fn in_active(&mut self, case: u8){
        self.active = case;
    }

}