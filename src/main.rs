use actix_web::delete;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Deserialize)]
struct ItemDescription {
    item: String,
}

#[get("/")]
async fn get_list(data: web::Data<Arc<Mutex<Vec<String>>>>) -> impl Responder {
    let list_items = data.lock().unwrap();
    println!("{:?}", list_items);
    let json_format = serde_json::to_string(&*list_items).unwrap();
    HttpResponse::Ok().body(json_format)
}

#[post("/add-item")]
async fn add_item(data: web::Data<Arc<Mutex<Vec<String>>>>, req_body: String) -> impl Responder {
    let mut list_items = data.lock().unwrap();
    let parsed_item: Result<ItemDescription, serde_json::Error> = serde_json::from_str(&req_body);
    match parsed_item {
        Ok(item_desc) => {
            list_items.push(item_desc.item);
        }
        Err(_error) => {
            return HttpResponse::BadRequest().body("Request body is not correct");
        }
    }
    HttpResponse::Ok().body(req_body)
}
#[delete("/delete-item")]
async fn delete_item(data: web::Data<Arc<Mutex<Vec<String>>>>, req_body: String) -> impl Responder {
    let mut list_items = data.lock().unwrap();
    let parsed_item: Result<ItemDescription, serde_json::Error> = serde_json::from_str(&req_body);
    match parsed_item {
        Ok(item_desc) => {
            for i in 1..list_items.len() {
                if list_items[i - 1] == item_desc.item {
                    list_items[i - 1] = list_items[i].clone();
                    list_items[i] = item_desc.item.clone();
                }
            }
            let item_popped: Option<String> = list_items.pop();
            match item_popped {
                Some(item_popped) => {
                    if item_popped == item_desc.item {
                        println!("Item deleted --> {}", item_popped);
                    } else {
                        list_items.push(item_popped);
                        return HttpResponse::NotFound()
                            .body(String::from("ERROR: item not found --> ") + &item_desc.item);
                    }
                }
                None => {
                    return HttpResponse::NotFound()
                        .body(String::from("ERROR: item not found --> ") + &item_desc.item);
                }
            }
        }
        Err(_error) => {
            return HttpResponse::BadRequest().body("Request body is not correct");
        }
    }
    HttpResponse::Ok().body(req_body)
}
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let to_do_list = Arc::new(Mutex::new(Vec::<String>::new()));
    HttpServer::new(move || {
        let to_do_list_clone = Arc::clone(&to_do_list);
        App::new()
            .app_data(web::Data::new(to_do_list_clone))
            .service(get_list)
            .service(add_item)
            .service(delete_item)
            .route("/hello", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
