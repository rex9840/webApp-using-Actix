use actix_web::{get,web,post,App,HttpResponse,HttpServer,Responder};



#[get("/")]
async fn hello()-> impl Responder
{ 
    return HttpResponse::Ok().body("Hello World");
}

#[post("/echo")]
async fn echo(req_body:String)-> impl Responder
{ 
    return HttpResponse::Ok().body(req_body);
}

async fn manual_hello_()-> impl Responder
{ 
    return HttpResponse::Ok().body("Hey"); 
}


#[actix_web::main]
async fn main()->std::io::Result<()>
{ 
    HttpServer::new(||{
            App::new()
            .service(hello)
            .service(echo)
            .route("/hey",web::get().to(manual_hello_))
        }
    )
    .bind(("localhost",8080))?
    .run()
    .await
}