use chrono::prelude::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

mod schema {
    table! {
        messages (id) {
            id -> Text,
            name -> Text,
            content -> Text,
            time -> BigInt,
        }
    }
}

use self::schema::messages;

#[table_name = "messages"]
#[derive(Serialize, Deserialize, Queryable, Insertable, PartialEq, Debug)]
pub struct Message {
    pub id: String,
    pub name: String,
    pub content: String,
    pub time: i64,
}

impl Message {
    pub fn insert(
        conn: &diesel::SqliteConnection,
        name: &str,
        content: &str,
    ) -> Result<Message, &'static str> {
        let message = Message {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            content: content.to_string(),
            time: Utc::now().timestamp_millis(),
        };
        let result = diesel::insert_into(messages::table)
            .values(&message)
            .execute(conn);
        if result.is_ok() {
            Ok(message)
        } else {
            Err("Insertion failed")
        }
    }

    pub fn get(conn: &diesel::SqliteConnection, id: &str) -> Result<Message, &'static str> {
        if let Ok(m) = messages::table.find(id).first(conn) {
            Ok(m)
        } else {
            Err("Get failed")
        }
    }

    pub fn delete(conn: &diesel::SqliteConnection, id: &str) -> Result<(), &'static str> {
        let message = messages::table.filter(messages::id.eq(id));
        if diesel::delete(message).execute(conn).is_ok() {
            Ok(())
        } else {
            Err("Deletion failed")
        }
    }

    pub fn update(
        conn: &diesel::SqliteConnection,
        id: &str,
        content: &str,
    ) -> Result<(), &'static str> {
        let message = messages::table.filter(messages::id.eq(id));
        let result = diesel::update(message)
            .set(messages::content.eq(content))
            .execute(conn);
        if result.is_ok() {
            Ok(())
        } else {
            Err("Update failed")
        }
    }

    pub fn all(conn: &diesel::SqliteConnection) -> Result<Vec<Message>, &'static str> {
        let result = messages::table
            .order(messages::time.desc())
            .into_boxed()
            .load::<Message>(conn);
        if let Ok(m) = result {
            Ok(m)
        } else {
            Err("Get all failed")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use diesel::result::Error;
    const NAME: &str = "Hoge";
    const CONTENT: &str = "fuga fuga!";
    const CONTENT2: &str = "fuga fuga!!";
    embed_migrations!();

    fn run_db() -> diesel::SqliteConnection {
        let conn = diesel::SqliteConnection::establish(":memory:").unwrap();
        embedded_migrations::run(&conn).unwrap();
        conn
    }

    #[test]
    fn test_insert() {
        let conn = run_db();
        conn.test_transaction::<_, Error, _>(|| {
            let result_message = Message::insert(&conn, NAME, CONTENT);
            assert!(!result_message.is_err());
            let inserted_message = result_message.unwrap();
            let message = Message::all(&conn).unwrap();
            assert_eq!(message.len(), 1);
            assert_eq!(message[0], inserted_message);
            Ok(())
        });
    }

    #[test]
    fn test_get() {
        let conn = run_db();
        conn.test_transaction::<_, Error, _>(|| {
            let _ = Message::insert(&conn, NAME, CONTENT);
            let result_message = Message::insert(&conn, NAME, CONTENT);
            let _ = Message::insert(&conn, NAME, CONTENT);
            assert!(!result_message.is_err());
            let inserted_message = result_message.unwrap();
            let message = Message::get(&conn, &inserted_message.id).unwrap();
            assert_eq!(message, inserted_message);
            Ok(())
        });
    }

    #[test]
    fn test_delete() {
        let conn = run_db();
        conn.test_transaction::<_, Error, _>(|| {
            let _ = Message::insert(&conn, NAME, CONTENT);
            let result_message = Message::insert(&conn, NAME, CONTENT);
            let _ = Message::insert(&conn, NAME, CONTENT);
            assert!(!result_message.is_err());
            let inserted_message = result_message.unwrap();
            assert!(!Message::delete(&conn, &inserted_message.id).is_err());
            let message = Message::get(&conn, &inserted_message.id);
            assert!(message.is_err());
            Ok(())
        });
    }

    #[test]
    fn test_update() {
        let conn = run_db();
        conn.test_transaction::<_, Error, _>(|| {
            let _ = Message::insert(&conn, NAME, CONTENT);
            let result_message = Message::insert(&conn, NAME, CONTENT);
            let _ = Message::insert(&conn, NAME, CONTENT);
            assert!(!result_message.is_err());
            let inserted_message = result_message.unwrap();
            assert!(!Message::update(&conn, &inserted_message.id, CONTENT2).is_err());
            let message = Message::get(&conn, &inserted_message.id).unwrap();
            assert_eq!(message.content, CONTENT2);
            Ok(())
        });
    }
}
