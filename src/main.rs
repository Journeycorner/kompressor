#![feature(proc_macro)]

#[macro_use]
extern crate stdweb;

use stdweb::js_export;
use stdweb::Reference;
use stdweb::web::Blob;
use stdweb::web::document;
use stdweb::web::FileReader;
use stdweb::web::FileReaderResult;
use stdweb::web::IBlob;

fn process(file_reader: FileReader) -> String {
    match file_reader.result() {
        Some(value) => match value {
            FileReaderResult::String(value) => value,
            _ => String::from("not a text"),
        }
        None => String::from("empty")
    }
}

#[js_export]
fn create_download_link(file_reader: FileReader) {
    let data = process(file_reader);
    js! {
        download(@{data});
    }
}

fn main() {
    stdweb::initialize();
    stdweb::event_loop();
}
