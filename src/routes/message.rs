use super::{Message, MessagesDb};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MessageInfo {
    name: String,
    content: String,
}

#[rocket::get("/")]
pub fn all(conn: MessagesDb) -> Result<Json<Vec<Message>>, &'static str> {
    match Message::all(&conn) {
        Ok(all) => Ok(Json(all)),
        Err(e) => Err(e)
    }
}

#[rocket::get("/<id>")]
pub fn get(id: String, conn: MessagesDb) -> Result<Json<Message>, &'static str> {
    match Message::get(&conn, &id) {
        Ok(m) => Ok(Json(m)),
        Err(e) => Err(e),
    }
}

#[rocket::post("/", format = "application/json", data = "<message_json>")]
pub fn new(message_json: Json<MessageInfo>, conn: MessagesDb) -> Result<(), &'static str> {
    let message = message_json.into_inner();
    match Message::insert(&conn, &message.name, &message.content) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[rocket::delete("/<id>")]
pub fn delete(id: String, conn: MessagesDb) -> Result<(), &'static str> {
    match Message::delete(&conn, &id) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[rocket::put("/<id>", format = "application/json", data = "<content_json>")]
pub fn update(
    id: String,
    content_json: Json<String>,
    conn: MessagesDb,
) -> Result<(), &'static str> {
    let content = content_json.into_inner();
    match Message::update(&conn, &id, &content) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {
    use super::super::run_db_migrations;
    use super::*;
    use rocket::fairing::AdHoc;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;
    use std::fs;
    const NAME: &str = "HOGE";
    const CONTENT: &str = "FUGA";
    fn get_client() -> Client {
        let _ = fs::remove_file("sqlite.db");
        let rocket = rocket::ignite()
            .mount("/", rocket::routes![all, new, delete, update])
            .attach(MessagesDb::fairing())
            .attach(AdHoc::on_attach("Database Migrations", run_db_migrations));
        Client::new(rocket).expect("valid rocket instance")
    }

    fn post(client: &Client) {
        let json_body = format!(
            r#"{{
            "name": "{}",
            "content":"{}"
         }}"#,
            NAME, CONTENT
        );
        let res = client
            .post("/")
            .header(ContentType::JSON)
            .body(json_body)
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
    }

    #[test]
    fn test_new() {
        let client = get_client();
        post(&client);
        let mut res = client.get("/").dispatch();
        assert_eq!(res.status(), Status::Ok);
        let body = res.body().unwrap().into_string().unwrap();
        let body: Vec<Message> = serde_json::from_str(&body).unwrap();
        assert_eq!(body.len(), 1);
        assert_eq!(body[0].name, NAME);
        assert_eq!(body[0].content, CONTENT);
        let _ = fs::remove_file("sqlite.db");
    }

    #[test]
    fn test_delete() {
        let client = get_client();
        post(&client);
        let mut res = client.get("/").dispatch();
        let body = res.body().unwrap().into_string().unwrap();
        let body: Vec<Message> = serde_json::from_str(&body).unwrap();
        assert_eq!(body.len(), 1);
        assert_eq!(res.status(), Status::Ok);

        let res = client.delete(format!("/{}", body[0].id)).dispatch();
        assert_eq!(res.status(), Status::Ok);

        let mut res = client.get("/").dispatch();
        assert_eq!(res.status(), Status::Ok);
        let body = res.body().unwrap().into_string().unwrap();
        let body: Vec<Message> = serde_json::from_str(&body).unwrap();
        assert_eq!(body.len(), 0);
        let _ = fs::remove_file("sqlite.db");
    }
}
