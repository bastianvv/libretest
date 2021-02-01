#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate env_logger;

use actix_web::{App, HttpServer, middleware};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod db;
mod schema;
mod requirements;
mod error_handler;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    db::init();

    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(||{
        App::new()
            .configure(requirements::init_routes)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set HOST in .env");
            let port = env::var("PORT").expect("Please set PORT in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}
