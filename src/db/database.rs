use  std::{fmt::Error};
use chrono::prelude::*;
use std::sync::{Arc,Mutex};

use crate::{models::todo::{Todo}}; 


pub struct Database
{ 
    pub todos: Arc<Mutex<Vec<Todo>>>,
}


impl Database
{
    pub fn new()->Self
    { 
        let todo =Arc::new(Mutex::new(Vec::from([])));
        return Database{todos:todo};
    
    }  

    pub fn create_todo(&self,todo:Todo)->Result<Todo,Error>
    {
            let mut todos = self.todos.lock().unwrap();
            let id = uuid::Uuid::new_v4().to_string(); 
            let created_at = Utc::now();
            let updated_at = Utc::now();
            let todo = Todo{
                id:Some(id),
                created_at:Some(created_at),
                updated_at:Some(updated_at),
                ..todo
                //struct update syntax where the remaning value is upated and initiated form todo 
    };

    todos.push(todo.clone());
    return Ok(todo);
    } 

    pub fn get_todos(&self)->Vec<Todo>
    { 
        let todos = self.todos.lock().unwrap();
        return todos.clone(); 
        
    }

    pub fn get_todos_by_id(&self,id:&str)->Option<Todo>
    { 
        let todos =self.todos.lock().unwrap();
        return todos.iter().find(|todo| todo.id == Some(id.to_string())).cloned();
    }

    pub fn update_todo_by_id(&self,id:&str,update_todo:Todo)->Option<Todo>
    { 
         let mut todos = self.todos.lock().unwrap();
         let update_at = Utc::now(); 
         let todo = Todo{ 
            id : Some(id.to_string()),
            updated_at:Some(update_at),
            ..update_todo
         };
         let index = todos.iter().position(|todo|{todo.id ==Some(id.to_string())})?;
         todos[index] = todo.clone();
         return Some(todo)
    }

    pub fn delete_todo_by_id(&self,id:&str)->Option<Todo>
    {
        let mut todos = self.todos.lock().unwrap();
        let index = todos.iter().position(|todo|{todo.id==Some(id.to_string())})?;
        return  Some(todos.remove(index));
    }

}
