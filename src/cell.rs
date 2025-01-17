// pub const NUM_SIDE_TOTAL : usize = 6;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    Live,
    Dead
}
impl Default for Status {
    fn default() -> Status {
        Status::Dead
    }
}

#[derive(Debug, Default)]
pub struct Cell {
    status: Status,
    position: (i32, i32)
}

impl Cell {
    //  init
    pub fn new() -> Cell {
        Cell {
            status: Status::Dead,
            position: (0, 0)
        }
    }
    //  geter
    pub fn get_status(&mut self) -> Status {
        self.status
    }
    pub fn get_x(&mut self) -> i32 {
        self.position.0
    }
    pub fn get_y(&mut self) -> i32 {
        self.position.1
    }
    //  
    pub fn set_x(&mut self, x: i32) {
        self.position.0 = x;
    }
    pub fn set_y(&mut self, y: i32) {
        self.position.1 = y;
    }
    pub fn change_status(&mut self) {
        if self.status == Status::Dead {
            self.status = Status::Live;
        }
        else {
            self.status = Status::Dead;
        }
    }
}