use actix_web::{get, web, HttpResponse, Responder};
use ignore::WalkBuilder;
use serde::{Deserialize, Serialize};
use std::{fs, io::Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct Frontmatter {
    title: String,
    file_name: String,
    description: String,
    posted: String,
    tags: Vec<String>,
    author: String,
    estimated_reading_time: u32,
    order: u32,
}

fn find_all_frontmatters() -> Result<Vec<Frontmatter>, std::io::Error> {
    let mut t = ignore::types::TypesBuilder::new();
    t.add_defaults();
    let toml = match t.select("toml").build() {
        Ok(t) => t,
        Err(e) => {
            println!("{:}", e);
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "could not build toml file type matcher",
            ));
        }
    };

    let file_walker = WalkBuilder::new("./posts").types(toml).build();
    let mut frontmatters = Vec::new();
    for frontmatter in file_walker {
        match frontmatter {
            Ok(fm) => {
                if fm.path().is_file() {
                    let fm_content = fs::read_to_string(fm.path())?;
                    let frontmatter: Frontmatter = match toml::from_str(&fm_content) {
                        Ok(f) => f,
                        Err(e) => {
                            println!(
                                "could not parse frontmatter for {:}: {:}",
                                fm.path().display(),
                                e,
                            );
                            return Err(Error::new(
                                std::io::ErrorKind::Other,
                                "could not parse frontmatter",
                            ));
                        }
                    };

                    frontmatters.push(frontmatter);
                }
            }
            Err(e) => {
                println!("{:}", e); // we're just going to print the error for now
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    "could not locate frontmatter",
                ));
            }
        }
    }

    Ok(frontmatters)
}

#[get("/")]
pub async fn index(templates: web::Data<tera::Tera>) -> impl Responder {
    let mut context = tera::Context::new();

    let mut frontmatters = match find_all_frontmatters() {
        Ok(fm) => fm,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!</p>");
        }
    };
    frontmatters.sort_by(|a, b| b.order.cmp(&a.order));

    context.insert("posts", &frontmatters);

    // temporary - move to default function later

    context.insert("nav_site_href", "/");
    context.insert("root_uri", "/");
    context.insert("linkedin_uri", "linkedin");
    context.insert("github_uri", "github");
    context.insert("resume_uri", "resume");
    context.insert("blog_uri", "blog");
    context.insert("domain_name", "erik-engelhardt.com");
    context.insert("internet_handle", "Erik Engelhardt");
    context.insert("web_sep", " | ");
    context.insert("my_email", "erik.raik.engelhardt@gmail.com");
    context.insert("full_name", "Erik Engelhardt");
    context.insert("title", "This is a test.");

    match templates.render("home.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!</p>")
        }
    }
}
