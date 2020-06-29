#[macro_use]
extern crate log;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use dotenv::dotenv;
use listenfd::ListenFd;
use rustls::{
    internal::pemfile::{certs, rsa_private_keys},
    NoClientAuth, ServerConfig,
};
use sqlx::SqlitePool;
use std::{env, fs::File, io::BufReader};

mod texts;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        r#"Please visit the `/api/v1/texts` endpoint for the main content!

Available routes:
    GET /texts -> array of entry objects consisting of: id, category, title, description
    GET /texts/{id} -> text object consisting of: id, category, title, content"#,
    )
}

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open(env::var("CERTIFICATE")?)?);
    let key_file = &mut BufReader::new(File::open(env::var("CERTIFICATE_KEY")?)?);
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0))?;

    let mut listenfd = ListenFd::from_env();

    let database_url = env::var("DATABASE_URL")?;
    let db_pool = SqlitePool::new(&database_url).await?;

    let mut server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .service(index)
            .configure(texts::init)
    });

    server = if let Some(listener) = listenfd.take_tcp_listener(0)? {
        server.listen(listener)?
    } else {
        let host = env::var("HOST")?;
        let port = env::var("PORT")?;

        server.bind_rustls(format!("{}:{}", host, port), config)?
    };

    info!("Starting the server!");

    server.run().await?;

    Ok(())
}
