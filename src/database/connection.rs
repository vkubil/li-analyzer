use diesel::{Connection, SqliteConnection};

use crate::config::CONFIG;

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish(&CONFIG.database.url)
        .expect("Negalėjo prisijungti prie duomenų bazės")
}
