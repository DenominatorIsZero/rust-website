use actix_web::{get, web, HttpResponse, Responder};

#[get("/publications")]
pub async fn publications(templates: web::Data<tera::Tera>) -> impl Responder {
    let context = tera::Context::new();
    match templates.render("publications.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!</p>")
        }
    }
}
