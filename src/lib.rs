use actix_files::Files; // new line
use actix_web::{dev::Server, middleware, web, App, HttpServer};
use std::net::TcpListener;
use tera::Tera;
pub mod handlers;

#[macro_use]
extern crate lazy_static;
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

pub fn start_blog(listener: TcpListener) -> Result<Server, std::io::Error> {
    let srv = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(TEMPLATES.clone()))
            .wrap(middleware::Logger::default()) // enable logger
            .service(Files::new("/static", "static/").use_last_modified(true)) // new line
            .service(handlers::index)
            .service(handlers::blog)
            .service(handlers::publications)
            .service(handlers::post)
            .service(handlers::privacy)
    })
    .listen(listener)?
    .run();

    Ok(srv)
}
