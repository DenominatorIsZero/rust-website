use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, str::FromStr};

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
    fn new(demo_name: &str) -> anyhow::Result<Self> {
        let frontmatter = match extract_frontmatter(demo_name) {
            Ok(s) => s,
            Err(e) => {
                println!("{e:?}");
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

fn extract_frontmatter(demo_name: &str) -> anyhow::Result<Demo> {
    let frontmatter_input =
        match fs::read_to_string(format!("./demos/{demo_name}/demo_frontmatter.toml")) {
            Ok(s) => s,
            Err(e) => {
                println!("{e:?}");
                return Err(e.into());
            }
        };

    let frontmatter = match toml::from_str(&frontmatter_input) {
        Ok(fm) => fm,
        Err(e) => {
            println!("{e:?}");
            return Err(anyhow::anyhow!("could not parse demo frontmatter: {e}"));
        }
    };
    Ok(frontmatter)
}

fn check_file_exists(file_path: &str) -> bool {
    // Remove leading slash and prepend with static/ for actual file check
    let actual_path = if file_path.starts_with("/static/") {
        format!(".{file_path}")
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
            println!("{e:?}");
            return HttpResponse::NotFound()
                .content_type("text/html")
                .body("<p>Could not find demo - sorry!</p>");
        }
    };

    context.insert("demo_context", &demo_context);

    match tmpl.render("demo.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            println!("{e:?}");
            HttpResponse::NotFound()
                .content_type("text/html")
                .body("<p>Could not render demo - sorry!</p>")
        }
    }
}

fn find_all_demos() -> anyhow::Result<Vec<DemoContext>> {
    let demos_dir = Path::new("./demos");
    let mut demo_contexts = Vec::new();

    // Read directory entries
    for entry in fs::read_dir(demos_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Only process directories
        if path.is_dir() {
            if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                // Try to create DemoContext for this directory
                match DemoContext::new(dir_name) {
                    Ok(demo_context) => {
                        demo_contexts.push(demo_context);
                    }
                    Err(e) => {
                        println!("Skipping directory '{dir_name}': {e:?}");
                        // Continue processing other directories instead of failing completely
                    }
                }
            }
        }
    }

    demo_contexts.sort_by(|a, b| b.demo.date.cmp(&a.demo.date));

    Ok(demo_contexts)
}

#[get("/demos")]
pub async fn demos(templates: web::Data<tera::Tera>) -> impl Responder {
    let mut context = tera::Context::new();
    let demos = find_all_demos().unwrap();
    context.insert("demos", &demos);
    match templates.render("demos.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            println!("{e:?}");
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!</p>")
        }
    }
}
