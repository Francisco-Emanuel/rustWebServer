pub mod schema;
pub mod db_models;
mod db_utils;
mod services;


//main services
use actix::SyncArbiter;
use actix_files as fs;
use actix_web::{ web::Data, App, HttpServer };
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection
};
use dotenvy::dotenv;
use std::env;

//my services
use services::{index, pong};
use db_utils::{get_pool, AppState, DbActor};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {db: db_addr.clone()}))
            .service(index)
            .service(pong)
            .service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 42069))?
    .run()
    .await
}