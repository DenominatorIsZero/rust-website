use std::net::TcpListener;

use personal_website::start_blog;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:8080")?;
    start_blog(listener)?.await?;
    Ok(())
}
