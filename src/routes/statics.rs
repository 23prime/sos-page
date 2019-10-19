use std::path::{Path, PathBuf};

use rocket::get;
use rocket::response::NamedFile;

#[get("/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    return NamedFile::open(Path::new("static/").join(file)).ok();
}
