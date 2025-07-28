use actix_web::{get, web, HttpResponse, Responder};
use ignore::WalkBuilder;
use serde::{Deserialize, Serialize};
use std::{fs, io::Error, path::Path, str::FromStr};

#[derive(Serialize, Deserialize, Debug)]
pub struct Demo {
    pub title: String,
    pub description: String,
    pub instructions: String,
    pub status: String,
    pub date: String,
    pub wasm_path: String,
    pub repository: String,
    pub thumbnail: String,
    pub demo_type: String,
    pub tech_stack: Vec<String>,
    pub keywords: Vec<String>,
    pub model_size: String,
    pub browser_requirements: String,
    pub performance_notes: String,
}

#[derive(Serialize, Debug)]
pub struct DemoContext {
    pub demo: Demo,
    pub wasm_exists: bool,
    pub thumbnail_exists: bool,
    pub name: String,
}

impl DemoContext {
    fn new(demo_name: &str) -> Result<Self, Error> {
        let frontmatter = match extract_frontmatter(&demo_name) {
            Ok(s) => s,
            Err(e) => {
                println!("{:?}", e);
                return Err(e);
            }
        };

        let wasm_exists = check_file_exists(&frontmatter.wasm_path);
        let thumbnail_exists = check_file_exists(&frontmatter.thumbnail);

        Ok(DemoContext {
            demo: frontmatter,
            wasm_exists,
            thumbnail_exists,
            name: String::from_str(demo_name).unwrap(),
        })
    }
}

fn extract_frontmatter(demo_name: &str) -> Result<Demo, Error> {
    let frontmatter_input =
        match fs::read_to_string(format!("./demos/{}/demo_frontmatter.toml", demo_name)) {
            Ok(s) => s,
            Err(e) => {
                println!("{:?}", e);
                return Err(e);
            }
        };

    let frontmatter = match toml::from_str(&frontmatter_input) {
        Ok(fm) => fm,
        Err(e) => {
            println!("{:?}", e);
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "could not parse demo frontmatter",
            ));
        }
    };
    Ok(frontmatter)
}

fn check_file_exists(file_path: &str) -> bool {
    // Remove leading slash and prepend with static/ for actual file check
    let actual_path = if file_path.starts_with("/static/") {
        format!(".{}", file_path)
    } else {
        file_path.to_string()
    };
    Path::new(&actual_path).exists()
}

#[get("/demos/{demo_name}")]
pub async fn demo(tmpl: web::Data<tera::Tera>, demo_name: web::Path<String>) -> impl Responder {
    let mut context = tera::Context::new();

    let demo_context = match DemoContext::new(&demo_name) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::NotFound()
                .content_type("text/html")
                .body("<p>Could not find demo - sorry!</p>");
        }
    };

    context.insert("demo_context", &demo_context);

    match tmpl.render("demo.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::NotFound()
                .content_type("text/html")
                .body("<p>Could not render demo - sorry!</p>");
        }
    }
}

fn find_all_demos() -> Result<Vec<DemoContext>, std::io::Error> {
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

    let file_walker = WalkBuilder::new("./demos").types(toml).build();
    let mut frontmatters = Vec::new();
    for frontmatter in file_walker {
        match frontmatter {
            Ok(fm) => {
                if fm.path().is_file() {
                    let fm_content = fs::read_to_string(fm.path())?;
                    let demos: DemoContext = match toml::from_str(&fm_content) {
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

                    frontmatters.push(demos);
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

    frontmatters.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(frontmatters)
}

#[get("/demos")]
pub async fn demos(templates: web::Data<tera::Tera>) -> impl Responder {
    let mut context = tera::Context::new();
    let demos = find_all_demos().unwrap();
    context.insert("demos", &demos);
    match templates.render("demos.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!</p>")
        }
    }
}
