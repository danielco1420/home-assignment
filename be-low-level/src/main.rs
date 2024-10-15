mod router;
mod utils;

use actix_web::{App, HttpServer};
use router::router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 4000;
    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .configure(router)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
