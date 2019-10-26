
use rocket::data::Data;
use std::io;

#[post("/upload", data = "<file>")]
pub fn upload(file: Data) -> io::Result<String> {
    file.stream_to_file("/tmp/upload.txt").map(|n| n.to_string())
}