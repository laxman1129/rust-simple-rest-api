use actix_web::{App, HttpServer, Responder, get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Error;
use std::sync::{Arc, Mutex};

/////////////////////////
// Models start
/////////////////////////
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CreatedUser {
    id : u32,
    name: String,
}

/////////////////////////
// Models end
/////////////////////////

////////////////////////
// Database type alias for shared state start
////////////////////////
type UserDb = Arc<Mutex<HashMap<u32, User>>>;
//////////////////////////
// Database type alias for shared state end
/////////////////////////


////////////////////////
// Handlers start
////////////////////////


// simple greeting endpoint
#[get("/greet")]
async fn greet() -> impl Responder {
    "Hello, world!".to_string()
}

// endpoint with a path parameter of type String
#[get("/hello/{user}")]
async fn hello(user_name: web::Path<String>) -> impl Responder {
    format!("Hello, {}!", user_name)
}

// endpoint with a path parameter of type u32
#[get("/get-user/{id}")]
async fn get_user(user_id: web::Path<u32>) -> impl Responder {
    format!("Hello, id {}!", user_id)
}

/// endpoint to post a new user  
/// 
/// It accepts a JSON body with a `name` field  
/// 
/// db is a shared state of type `UserDb` which is a Mutex-wrapped HashMap  
/// 
/// It returns a JSON response with the created user's ID and name  
#[post("/post-user")]
async fn post_user(
    user_data: web::Json<User>,
    db: web::Data<UserDb>
) -> impl Responder {
    
    
    let mut db = db.lock().unwrap(); // Lock the mutex to access the shared state
    let new_id = db.keys().max().unwrap_or(&0) +1; // Generate a new ID by finding the max key and adding 1
    let name = user_data.name.clone();
    db.insert(new_id, user_data.into_inner());
    
    println!("User {} with ID {} added successfully!", name, new_id);
    
    // HttpResponse::Created().json(db.get(&new_id).unwrap())
    HttpResponse::Created().json(CreatedUser{
        id: new_id,
        name,
    })
}

/// endpoint to get a user by ID
/// It accepts a path parameter `id` of type `u32`
/// It returns a JSON response with the user data if found, or a 404 Not Found response if not found
#[get("/user/{id}")]
async fn user(
    user_id: web::Path<u32>,
    db: web::Data<UserDb>
) -> Result<impl Responder, Error> {
    // HttpResponse::Created().json(db.get(&user_id).cloned()).unwrap()
    // let result = db.get(&user_id).unwrap();
    // println!("Result for , id {} is {:?}", user_id, result);
    // HttpResponse::Created().json(result); // this results in panic
    
    let db = db.lock().unwrap();
    match db.get(&user_id) {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Ok(HttpResponse::NotFound().body("User not found")),
    }
}

////////////////////////
// Handlers end
////////////////////////

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    let address = "127.0.0.1";
    println!("Starting server at http://{}:{}", address, port);

    let user_db: UserDb = Arc::new(Mutex::new(HashMap::new()));
    HttpServer::new(move || {
        let app_data = web::Data::new(user_db.clone());
        App::new()
            .app_data(app_data)
            .service(greet)
            .service(hello)
            .service(get_user)
            .service(post_user)
            .service(user)
    })
    .bind((address, port))?
    .workers(2) // Set the number of worker threads
    .run()
    .await
}
