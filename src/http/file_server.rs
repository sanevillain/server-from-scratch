use super::{request::Request, server::Handler};
use std::{fs, io, path::Path, str};

#[derive(Clone)]
pub struct FileServer {
    path: &'static str,
}

impl FileServer {
    pub fn new(path: &'static str) -> Self {
        Self { path }
    }

    fn build_page(path: String, filenames: Vec<String>) -> String {
        const FIRST_HALF: &str = "<!DOCTYPE html><html><head><title>Hello</title></head><body>";
        const SECOND_HALF: &str = "</body></html>\r\n\r\n";

        let mut page: Vec<String> = vec![];

        page.push(FIRST_HALF.to_string());
        page.extend(FileServer::build_body(path, filenames));
        page.push(SECOND_HALF.to_string());
        page.join("")
    }

    fn build_body(path: String, filenames: Vec<String>) -> Vec<String> {
        let mut list = vec![];
        list.push(format!("<h3>{}</h3>", path));
        list.push(String::from("<ul>"));
        list.extend(filenames.iter().map(|filename| {
            format!(
                "<li><a href=\"{}{}{}\">{}</a></li>",
                path,
                if path != "/" { "/" } else { "" },
                filename,
                filename
            )
        }));
        list.push(String::from("</ul>"));
        list
    }
}

impl Handler for FileServer {
    fn serve_http(&self, req: Request) -> io::Result<Vec<u8>> {
        let path = format!(".{}", req.url.path);
        let path = Path::new(&path);

        if path.is_file() {
            return fs::read(path);
        }

        if !path.is_dir() {
            return Ok(String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n").into_bytes());
        }

        let links = fs::read_dir(path)?
            .filter_map(|entry| match entry {
                Ok(entry) => Some(
                    entry
                        .path()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned(),
                ),
                _ => None,
            })
            .collect::<Vec<String>>();

        let response_page = FileServer::build_page(req.url.path.to_owned(), links);
        Ok(response_page.into_bytes())
    }
}
