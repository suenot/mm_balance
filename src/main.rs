use actix_web::{web, App, HttpServer, Responder};
use mongodb::{options::ClientOptions, Client, bson::doc};
use std::io;
use dotenv::dotenv;
use std::env;

async fn get_balance(user_id: web::Path<String>) -> impl Responder {
  // Load the .env file
  dotenv().ok();
  // print name of function
  println!("get_balance");
  // print MONGODB_URL
  println!("MONGODB_URL: {:?}", env::var("MONGODB_URL").expect("MONGODB_URL must be set"));
  let user_id = user_id.into_inner();

  // Get the MongoDB connection string from the environment variable
  let mongodb_url = env::var("MONGODB_URL").expect("MONGODB_URL must be set");

  // Parse a connection string into an options struct.
  let mut client_options = ClientOptions::parse(&mongodb_url).await.unwrap();

  // Manually set an option.
  client_options.app_name = Some("My App".to_string());

  // Get a handle to the deployment.
  let client = Client::with_options(client_options).unwrap();

  // Get a handle to a collection in the database.
  // let collection = client.database("test").collection("users");
  let collection: mongodb::Collection<bson::document::Document> = client.database("test").collection("users");
  println!("collection: {:?}", collection);

  // Query the database for a user with the provided id.
  let filter = doc! { "id": user_id.clone() };
  let user = collection.find_one(filter, None).await.unwrap();

  match user {
    Some(user) => user.get("tokens").and_then(|b| b.as_i32()).unwrap_or_default().to_string(),
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