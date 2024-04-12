use rocket::serde::Serialize;
use diesel::{self, prelude::*};

mod schema {
    table! {
        waitlist_entries (email) {
            email -> Text,
            first_name -> Text,
            last_name -> Text,
            notes -> Text,
        }
    }
}

use self::schema::waitlist_entries;

use crate::DbConn;

#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = waitlist_entries)]
pub struct WaitlistEntry{
    #[serde(skip_deserializing)]
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub notes: String,
}

impl WaitlistEntry {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<WaitlistEntry>> {
        conn.run(|c| {
            waitlist_entries::table.order(waitlist_entries::last_name.desc()).load::<WaitlistEntry>(c)
        }).await
    }

    /// Returns the number of affected rows: 1.
    pub async fn insert(entry: WaitlistEntry, conn: &DbConn) -> QueryResult<usize> {
        conn.run(|c| {
            diesel::insert_into(waitlist_entries::table).values(entry).execute(c)
        }).await
    }

    /// Returns the number of affected rows: 1.
    pub async fn delete_with_email(email: String, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| diesel::delete(waitlist_entries::table)
            .filter(waitlist_entries::email.eq(email))
            .execute(c))
            .await
    }

    /// Returns the number of affected rows.
    #[cfg(test)]
    pub async fn delete_all(conn: &DbConn) -> QueryResult<usize> {
        conn.run(|c| diesel::delete(waitlist_entries::table).execute(c)).await
    }
}
