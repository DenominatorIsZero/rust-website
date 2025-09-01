use actix_files::Files; // new line
use actix_web::middleware::DefaultHeaders;
use actix_web::{dev::Server, http::header, middleware, web, App, HttpServer};
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
                println!("Parsing error(s): {e}");
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

pub fn start_server(listener: TcpListener) -> anyhow::Result<Server> {
    let srv = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(TEMPLATES.clone()))
            .wrap(middleware::Logger::default()) // enable logger
            .wrap(
                DefaultHeaders::new()
                    // Content Security Policy - strict security with WASM support
                    .add((
                        header::CONTENT_SECURITY_POLICY,
                        "default-src 'self'; script-src 'self' https://eu.i.posthog.com https://eu-assets.i.posthog.com 'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data:; connect-src 'self' https://eu.i.posthog.com https://eu-assets.i.posthog.com; font-src 'self'; object-src 'none'; base-uri 'self'; form-action 'self'; frame-src 'self'; upgrade-insecure-requests"
                    ))
                    // Prevent external clickjacking but allow same-origin iframes (for AI demos)
                    .add((header::X_FRAME_OPTIONS, "SAMEORIGIN"))
                    // Prevent MIME type sniffing
                    .add((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
                    // Control referrer information for privacy
                    .add((header::REFERRER_POLICY, "strict-origin-when-cross-origin"))
                    // Strict Transport Security for HTTPS-only sites
                    .add((header::STRICT_TRANSPORT_SECURITY, "max-age=31536000; includeSubDomains"))
            )
            .service(Files::new("/static", "static/").use_last_modified(true)) // new line
            .service(handlers::index)
            .service(handlers::blog)
            .service(handlers::publications)
            .service(handlers::demos)
            .service(handlers::post)
            .service(handlers::demo)
            .service(handlers::privacy)
    })
    .listen(listener)?
    .run();

    Ok(srv)
}
