use serde::Serialize;

#[derive(Serialize, Queryable, Debug)]
pub struct Message {
    id: i32,
    actor: i32,
    text: String,
}
