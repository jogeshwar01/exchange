use actix_web::{
    web::{self, scope},
    App, HttpResponse, HttpServer,
};
use confik::{Configuration as _, EnvSource};
use dotenvy::dotenv;
use routes::user;

pub mod config;
pub mod routes;
pub mod types;
use crate::config::RouterConfig;
use crate::routes::order;
use crate::types::app::AppState;

use redis::RedisManager;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();

    let config = RouterConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    let app_state = web::Data::new(AppState {
        redis_connection: RedisManager::new().await.unwrap(),
    });

    let server = HttpServer::new(move || {
        App::new().service(
            scope("/api/v1")
                .app_data(app_state.clone())
                .service(web::scope("/health").route("", web::get().to(HttpResponse::Ok))) // GET /api/v1/ping
                .service(web::scope("/users").route("", web::post().to(user::create_user))) // POST /api/v1/users
                .service(
                    web::scope("/orders")
                        .route("", web::post().to(order::execute_order)) // POST /orders
                        .route("", web::delete().to(order::cancel_order)) // DELETE /orders
                        .route("/open", web::get().to(order::get_open_orders)), // GET /orders/open
                ),
        )
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
