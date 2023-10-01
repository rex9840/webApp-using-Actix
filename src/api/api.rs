use actix_web::{web,post,put,get,delete ,HttpResponse, http};

use crate::{models::todo::{Todo, self},db::database::Database}; 


#[post("/todos")]
pub async fn create_todo(db:web::Data<Database>,new_todo:web::Json<Todo>)->HttpResponse
{ 
    let todo = db.create_todo(new_todo.into_inner());
    //The new_todo.into_inner() call extracts the inner Todo value from the Json wrapper. The create_todo method creates a new Todo record in the database and returns the newly created Todo object.
    match todo
    { 
        Ok(todo)=> return HttpResponse::Ok().json(todo),
        Err(err)=> return HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[get("/todos")]
pub async fn get_todods(db:web::Data<Database>)->HttpResponse
{ 
    let todos = db.get_todos();
    return HttpResponse::Ok().json(todos);
}



#[get("/todos/{id}")]
pub async fn get_todos_by_id(db:web::Data<Database>,id: web::Path<String>) -> HttpResponse
{
     let todo = db.get_todos_by_id(&id);
    
    match todo
     { 
        Some(todo)=> return HttpResponse::Ok().json(todo), 
        None=> return HttpResponse::NotFound().body("Todo not found") 
     };
}

#[put("/todos/{id}")]
pub async fn update_todo_by_id(db:web::Data<Database>,id:web::Path<String>,update:web::Json<Todo>)->HttpResponse
{ 
    let get_todo = db.update_todo_by_id(&id,update.into_inner()); 
    
    match get_todo 
    { 
        Some(get_todo)=> return HttpResponse::Ok().json(get_todo),
        None => return HttpResponse::NotFound().body("Todo not found")
    };
}


#[delete("/todos/{id}")]
async fn delete_todo_by_id(db:web::Data<Database>,id:web::Path<String>)->HttpResponse
{
    let todo  = db.delete_todo_by_id(&id); 
    match todo
    { 
        Some(todo)=>return  HttpResponse::Ok().json(todo),
        None=> return HttpResponse::NotFound().body("Todo not found") 
    };
}


pub fn config(cfg:&mut web::ServiceConfig)
{ 
    cfg.service(
        web::scope("/api")
        .service(create_todo)
        .service(get_todods )
        .service(get_todos_by_id)
        .service(update_todo_by_id)
        .service(delete_todo_by_id)
    );
}
 
