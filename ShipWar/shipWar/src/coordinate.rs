pub struct Coordinate{
    pub x: u8,
    pub y: u8
}

impl Coordinate{
    pub fn new(x_user:u8,y_user:u8) -> Self{
        Self{ x: x_user, y: y_user }
    }
}
