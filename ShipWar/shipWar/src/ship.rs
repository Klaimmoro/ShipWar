use crate::cell::Cell;

pub struct Ship{
    pub position: Cell 
}

impl Ship{
    pub fn new(X: u8,Y:u8) -> Self{
        Self{
            position: Cell::new(X,Y)
        }
    }
}