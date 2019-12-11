
use server::config::{get_config, ServerConfig};

use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index(info: actix_web::web::Path<()>) -> impl Responder {
    "aeuae"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = get_config();

    let bind = format!("{}:{}", config.host, config.port);
    let bind_tls = format!("{}:{}", config.host, config.port_tls);
    println!("binding http to {}", bind);
    println!("binding https to {}", bind_tls);

    HttpServer::new(|| App::new().service(index))
        .bind(bind)?
        .start()
        .await
}
