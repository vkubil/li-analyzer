use diesel::RunQueryDsl;

use crate::database::{connection::establish_connection, schema::imported_graduates};

pub fn remove_all_imported() -> Result<usize, diesel::result::Error> {
    let conn = &mut establish_connection();

    diesel::delete(imported_graduates::table).execute(conn)
}
