use diesel::prelude::*;
use crate::schema::users::dsl::*;
use crate::DB_CONNECTION;
use crate::app::app1::models::Users;

pub fn get_all_users() -> Result<Vec<Users>, diesel::result::Error> {
    let mut conn = DB_CONNECTION.get().expect("❌ Failed to get DB connection from pool");

    let results = users
    .limit(10)
    .select(Users::as_select())
    .load::<Users>(&mut *conn)?;

    Ok(results)
}
