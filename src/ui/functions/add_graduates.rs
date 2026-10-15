use diesel::RunQueryDsl;

use crate::database::{
    connection::establish_connection, models::ImportedGraduate, schema::imported_graduates,
};

pub fn add_graduates(ids: Vec<String>) -> Result<usize, diesel::result::Error> {
    let conn = &mut establish_connection();

    let values = ids
        .iter()
        .map(|id| ImportedGraduate { id: id.to_string() })
        .collect::<Vec<ImportedGraduate>>();

    diesel::insert_into(imported_graduates::table)
        .values(values)
        // FIXME .on_conflict_do_nothing()
        .execute(conn)
}
