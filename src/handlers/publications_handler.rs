use actix_web::{get, web, HttpResponse, Responder};
use ignore::WalkBuilder;
use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Publication {
    title: String,
    authors: Vec<String>,
    r#abstract: String,
    date: String,
    publication_type: String,
    venue: String,
    doi: Option<String>,
    url: Option<String>,
    pdf_url: Option<String>,
    bibtex: Option<String>,
    keywords: Option<Vec<String>>,
    citation: Option<String>,
    thumbnail: Option<String>,
    language: Option<String>,
}

fn find_all_publications() -> anyhow::Result<Vec<Publication>> {
    let mut t = ignore::types::TypesBuilder::new();
    t.add_defaults();
    let toml = match t.select("toml").build() {
        Ok(t) => t,
        Err(e) => {
            println!("{e:}");
            return Err(anyhow::anyhow!(
                "could not build toml file type matcher: {e}"
            ));
        }
    };

    let file_walker = WalkBuilder::new("./publications").types(toml).build();
    let mut frontmatters = Vec::new();
    for frontmatter in file_walker {
        match frontmatter {
            Ok(fm) => {
                if fm.path().is_file() {
                    let fm_content = fs::read_to_string(fm.path())?;
                    let publication: Publication = match toml::from_str(&fm_content) {
                        Ok(f) => f,
                        Err(e) => {
                            println!(
                                "could not parse frontmatter for {}: {e}",
                                fm.path().display(),
                            );
                            return Err(anyhow::anyhow!("could not parse frontmatter: {e}"));
                        }
                    };

                    frontmatters.push(publication);
                }
            }
            Err(e) => {
                println!("{e:}"); // we're just going to print the error for now
                return Err(anyhow::anyhow!("could not locate frontmatter: {e}"));
            }
        }
    }

    frontmatters.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(frontmatters)
}

#[get("/publications")]
pub async fn publications(templates: web::Data<tera::Tera>) -> impl Responder {
    let mut context = tera::Context::new();
    let publications = match find_all_publications() {
        Ok(pubs) => pubs,
        Err(e) => {
            println!("{e:?}");
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!</p>");
        }
    };
    context.insert("publications", &publications);
    match templates.render("publications.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            println!("{e:?}");
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!</p>")
        }
    }
}
