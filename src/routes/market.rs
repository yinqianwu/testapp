use rocket::response::NamedFile;
use rocket_contrib::json::{JsonValue};

use std::path::{Path, PathBuf};
use std::fs::{File, read_dir};
use std::io::prelude::*;

#[allow(unused_variables)]
#[get("/markets/<name>")]
pub fn get_market(name: String) -> Option<NamedFile> {
    println!("pub fn article");
    NamedFile::open(Path::new("web/dist/index.html")).ok()
}
