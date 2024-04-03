use actix_web::{web, App, HttpServer, Responder};
use std::io;
use lazy_static::lazy_static;

#[derive(Clone)]
struct User {
  id: String,
  tokens: i32,
}

lazy_static! {
  static ref USERS: Vec<User> = vec![
    User { id: String::from("user1"), tokens: 100 },
    User { id: String::from("user2"), tokens: 200 },
    User { id: String::from("user3"), tokens: 300 },
  ];
}

async fn get_balance(user_id: web::Path<String>) -> impl Responder {
  let user_id = user_id.into_inner();
  let user = USERS.iter().find(|user| user.id == user_id).cloned();
  match user {
    Some(user) => user.tokens.to_string(),
    None => format!("No user found with id {}", user_id),
  }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .route("/balance/{user_id}", web::get().to(get_balance))
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}