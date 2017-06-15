#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::response::NamedFile;
use std::path::Path;
use std::path::PathBuf;

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).ok()
}

#[get("/<path..>")]
fn everything_else(path: PathBuf) -> Option<NamedFile> {
    match path.extension() {
        Some(_) => {
            NamedFile::open(Path::new("static/").join(path)).ok()
        },
        None => {
            NamedFile::open(Path::new("static/").join(path).join("index.html")).ok()
        }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, everything_else]).launch();
}
