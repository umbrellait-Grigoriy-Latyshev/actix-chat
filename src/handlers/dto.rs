use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct PostMessageDto {
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct PostMessageDtoResponse {
    ok: bool,
}

impl PostMessageDtoResponse {
    pub fn new() -> PostMessageDtoResponse {
        PostMessageDtoResponse { ok: true }
    }
}
