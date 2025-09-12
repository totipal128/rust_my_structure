use diesel::{prelude::Queryable, Selectable};
use serde::Serialize;

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name=crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users{
    pub id:i64,
    pub name:String,
    pub address:String,
    pub is_agent:bool,
}