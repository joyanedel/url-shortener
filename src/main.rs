use actix_web::{get, http::header, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use url_shortener::{
    core::{constants::SHORT_URL_ALPHABET, urls::create_url},
    persistence::{get_url_entry, store_url_entry, UrlEntry},
};

struct ServerState {
    database_connection: Pool<Postgres>,
    alphabet: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/postgres")
        .await
        .expect("Couldn't establish connection with sql database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ServerState {
                database_connection: pool.clone(),
                alphabet: SHORT_URL_ALPHABET.to_string(),
            }))
            .service(post_url)
            .service(redirect_from_short_url)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Deserialize)]
struct UrlRequest {
    long_url: String,
}

#[post("/urls")]
async fn post_url(data: web::Data<ServerState>, info: web::Json<UrlRequest>) -> impl Responder {
    let url_entry = loop {
        let short_url = match create_url(7, &data.alphabet) {
            None => continue,
            Some(v) => v,
        };

        if get_url_entry(&data.database_connection, &short_url)
            .await
            .is_some()
        {
            continue;
        }

        break UrlEntry {
            short_url,
            long_url: info.long_url.clone(),
        };
    };
    let short_url = url_entry.short_url.clone();

    let result = store_url_entry(&data.database_connection, url_entry).await;

    if result.is_ok() {
        HttpResponse::Ok().body(short_url)
    } else {
        HttpResponse::ServiceUnavailable().body("Sorry, we coiuldn't process your request now")
    }
}

#[get("/u/{short_url}")]
async fn redirect_from_short_url(
    path: web::Path<(String,)>,
    data: web::Data<ServerState>,
) -> impl Responder {
    let short_url = path.into_inner().0;
    let url_entry_result = get_url_entry(&data.database_connection, &short_url).await;

    if let Some(url_entry) = url_entry_result {
        // Redirect::to(url_entry.long_url).temporary()
        HttpResponse::TemporaryRedirect()
            .insert_header((header::LOCATION, url_entry.long_url))
            .finish()
    } else {
        HttpResponse::NotFound().body("The provided url doesn't exists in our system")
    }
}
