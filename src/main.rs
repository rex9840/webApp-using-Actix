use actix_web::{get,web,Responder,HttpResponse,HttpServer,Result,App};
use serde::Serialize;
mod api;
mod models; 
mod db;

#[derive(Serialize)]
pub struct Response {
        pub message: String
}


async fn not_found() -> Result<HttpResponse>{
        let response = Response {
            message: "Resource not found".to_string(),
        };
        Ok(HttpResponse::NotFound().json(response))
    }
    



#[actix_web::main]
async fn main() -> std::io::Result<()>
{ 
        let todo_db = db::database::Database::new();
        let app_data = web::Data::new(todo_db);
        //The move keyword is used before the closure || to indicate that the closure should take ownership of the variables it captures from the surrounding environment. 
        HttpServer::new(move ||{
                App::new()
                .app_data(app_data.clone())
                .configure(api::api::config)
                .default_service(web::route().to(not_found))
                .wrap(actix_web::middleware::Logger::default())
                
        })
        .bind(("localhost", 8080))?
        .run()
        .await
   
}


