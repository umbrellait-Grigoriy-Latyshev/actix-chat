use crate::schema::messages;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable, Debug)]
pub struct Message {
    id: i32,
    actor: i32,
    text: String,
    #[serde(with = "naive_serializer")]
    created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "messages"]
pub struct NewMessage {
    pub actor: i32,
    pub text: String,
}

// https://serde.rs/custom-date-format.html
mod naive_serializer {
    use chrono::NaiveDateTime;
    use serde::{self, Serializer};

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.timestamp();
        serializer.serialize_i64(s)
    }

    /* // Comment as not used at the moment
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let secs = s.parse::<i64>().unwrap();
        Ok(NaiveDateTime::from_timestamp(secs, 0u32))
    }
    */
}
