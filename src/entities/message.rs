use crate::schema::messages;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable, Debug)]
pub struct Message {
    id: i32,
    actor: i32,
    text: String,
}

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "messages"]
pub struct NewMessage {
    pub actor: i32,
    pub text: String,
}
