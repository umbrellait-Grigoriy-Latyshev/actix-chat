use serde::{ Serialize};

#[derive(Serialize, Debug)]
pub struct Message {
    id: u32,
    actor: u32,
    text: String,
}
