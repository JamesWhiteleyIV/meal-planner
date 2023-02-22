use actix_files::Files;
use actix_web::{get, http::header::ContentType, App, HttpResponse, HttpServer, Responder};

mod crud;

const DB_FILENAME: &str = "test.db";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = crud::get_connection_pool(DB_FILENAME).await;
    crud::create_tables(&pool).await.unwrap();
    crud::populate_tables(&pool).await.unwrap();

    HttpServer::new(|| {
        App::new()
            .service(get_recipes)
            .service(get_ingredients)
            .service(get_tags)
            .service(Files::new("/", "./web-ui/build").index_file("index.html"))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

#[get("/api/recipes")]
async fn get_recipes() -> impl Responder {
    let pool = crud::get_connection_pool(DB_FILENAME).await;
    let recipes = crud::recipe::read(&pool).await.unwrap();
    HttpResponse::Ok().json(recipes)
}

#[get("/api/ingredients")]
async fn get_ingredients() -> impl Responder {
    let pool = crud::get_connection_pool(DB_FILENAME).await;
    let ingredients = crud::ingredient::read(&pool).await.unwrap();
    HttpResponse::Ok().json(ingredients)
}

#[get("/api/tags")]
async fn get_tags() -> impl Responder {
    let pool = crud::get_connection_pool(DB_FILENAME).await;
    let tags = crud::tag::read(&pool).await.unwrap();
    HttpResponse::Ok().json(tags)
}
