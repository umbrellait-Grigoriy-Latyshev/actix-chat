use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TTT {
    sum: i32,
}


impl TTT {
    pub fn new (value: i32) -> TTT {
        TTT {sum: value}
    }
}