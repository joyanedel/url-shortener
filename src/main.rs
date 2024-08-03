use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

struct ServerState {
    database_connection: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("")
        .await
        .expect("Couldn't establish connection with sql database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ServerState {
                database_connection: pool.clone(),
            }))
            .service(post_url)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[post("/urls")]
async fn post_url() -> impl Responder {
    HttpResponse::Ok().body("URL Created succesfully")
}
