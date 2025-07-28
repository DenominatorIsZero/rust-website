use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::{fs, io::Error, path::Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct DemoFrontmatter {
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
    pub demo: DemoFrontmatter,
    pub wasm_exists: bool,
    pub thumbnail_exists: bool,
}

fn extract_frontmatter(demo_name: &str) -> Result<DemoFrontmatter, Error> {
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

    let frontmatter = match extract_frontmatter(&demo_name) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::NotFound()
                .content_type("text/html")
                .body("<p>Could not find demo - sorry!</p>");
        }
    };

    let wasm_exists = check_file_exists(&frontmatter.wasm_path);
    let thumbnail_exists = check_file_exists(&frontmatter.thumbnail);

    let demo_context = DemoContext {
        demo: frontmatter,
        wasm_exists,
        thumbnail_exists,
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