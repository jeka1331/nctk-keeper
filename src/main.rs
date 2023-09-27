use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_derive::Deserialize;
use std::sync::Mutex;


#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct NetworkDep {
    pub repo_url: Option<String>,

    pub name: String,
    pub public: bool,
    pub host: String,
    pub proxy: Option<String>,
    pub port: Option<i64>,
    pub ports: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct NetworkDeps {
    pub addresses: Vec<NetworkDep>
}

struct AppStateNetworkDeps {
    adresses: Mutex<Vec<NetworkDep>>
}


#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Ok!")
}

#[post("/address/add")]
async fn address_add(data: web::Json<NetworkDep>, state: web::Data<AppStateNetworkDeps>) -> impl Responder {
    println!("{:?}", data);
    let mut adresses = state.adresses.lock().unwrap();
    println!("{:?}", adresses);
    adresses.push(data.into_inner());
    println!("{:?}", adresses);
    
    "Success added!"
}

#[post("/address/addmany")]
async fn address_add_many(mut data: web::Json<NetworkDeps>, state: web::Data<AppStateNetworkDeps>) -> impl Responder {
    println!("{:?}", data);
    let mut adresses = state.adresses.lock().unwrap();
    println!("{:?}", adresses);
    
    adresses.append(&mut data.addresses);
    println!("{:?}", adresses);
    
    "Success added!"
}

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addresses = web::Data::new(AppStateNetworkDeps {
        adresses: Mutex::new(vec![])
    });
    HttpServer::new(move || {
        App::new()
            .app_data(addresses.clone())
            .service(health)
            .service(address_add)
            .service(address_add_many)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}