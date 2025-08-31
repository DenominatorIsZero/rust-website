use std::net::TcpListener;

use personal_website::start_blog;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let listener = TcpListener::bind("0.0.0.0:8080")?;
    start_blog(listener)?.await?;
    Ok(())
}
