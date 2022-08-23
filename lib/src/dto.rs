use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct PostMessageDtoResponse {
    ok: bool,
}

impl PostMessageDtoResponse {
    pub fn new(result: bool) -> PostMessageDtoResponse {
        PostMessageDtoResponse { ok: result }
    }
}

#[derive(Serialize, Debug)]
pub struct UserExistsDto {
    exists: bool,
}

impl UserExistsDto {
    pub fn new(result: bool) -> UserExistsDto {
        UserExistsDto { exists: result }
    }
}
