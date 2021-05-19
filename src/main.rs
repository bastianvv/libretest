extern crate openssl;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate env_logger;
extern crate openssl_probe;
use actix_web::{App, HttpServer, middleware};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;
use actix_redis::RedisSession;

mod db;
mod schema;
mod users;
mod requirements;
mod test_cases;
mod test_plans;
mod test_executions;
mod error_handler;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    openssl_probe::init_ssl_cert_env_vars();
    dotenv().ok();
    db::init();

    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let redis_host = env::var("REDIS_HOST").expect("Redis host not set");
    let redis_port = env::var("REDIS_PORT").expect("Redis port not set");

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move||{
        App::new()
            .wrap(RedisSession::new(format!("{}:{}", redis_host, redis_port), &[0; 32]))
            .wrap(IdentityService::new(
          CookieIdentityPolicy::new(&[0; 32])
              .name("users-cookie")
              .secure(false)))
            .configure(users::init_routes)
            .configure(requirements::init_routes)
            .configure(test_cases::init_routes)
            .configure(test_plans::init_routes)
            .configure(test_executions::init_routes)
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
