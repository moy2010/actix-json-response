use actix_web::{get, web, Result};
use actix_json_response::JsonResponse;
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
  name: String,
}

#[get("/a/{name}")]
async fn index(name: web::Path<String>) -> Result<JsonResponse<MyObj>> {
  let my_obj = MyObj {
    name: name.to_string(),
  };
  Ok(my_obj.into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  use actix_web::{App, HttpServer};

  HttpServer::new(|| App::new().service(index))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}