use super::{request::Request, response::Response, server::Handler, status::Status};
use std::{fs, io, path::Path, str};

#[derive(Clone)]
pub struct FileServer {
    path: &'static str,
}

impl FileServer {
    pub fn new(path: &'static str) -> Self {
        Self { path }
    }
}

impl FileServer {
    fn html_template(&self, body: String) -> String {
        format!(
            "<!DOCTYPE html>\
            <html lang=\"en\">\
            <head>\
                <meta charset=\"UTF-8\">\
                <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\
                <title>Hello</title>\
            </head>\
            <body>{}</body>\
            </html>",
            body
        )
    }

    fn build_body(&self, path: String, previous_path: String, filenames: Vec<String>) -> String {
        let mut path = &path[..];
        if path == "/" {
            path = "";
        }

        let header = format!("<h3>{}{}</h3>", self.path, path);
        let back = if !path.is_empty() {
            format!("<a href=\"{}\">Back</a>", previous_path)
        } else {
            format!("")
        };
        let lis = filenames
            .iter()
            .map(|filename| format!("<li><a href=\"{}{}\">{}</a></li>", path, filename, filename))
            .collect::<Vec<String>>();
        let ul = format!("<ul>{}</ul>", lis.join(""));

        format!("{}{}{}", header, back, ul)
    }
}

impl FileServer {
    fn get_parent_path(&self, parenth_path: &Path) -> String {
        let parent_path_str = parenth_path.to_str().unwrap_or("").to_string();

        if parent_path_str.starts_with(self.path) {
            let parenth_path = parent_path_str
                .strip_prefix(self.path)
                .expect("Couldn't trim previous path")
                .to_string();

            if !parenth_path.is_empty() {
                return parenth_path;
            }
        }

        "/".to_string()
    }

    fn get_filenames(&self, path: &Path) -> io::Result<Vec<String>> {
        let links = fs::read_dir(path)?
            .filter_map(|entry| match entry {
                Ok(entry) => Some(entry),
                _ => None,
            })
            .map(|entry| {
                entry
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .map(|filename| format!("/{}", filename))
            .collect::<Vec<String>>();

        Ok(links)
    }
}

impl Handler for FileServer {
    fn serve_http(&self, req: Request) -> io::Result<Response> {
        let req_url = req.url.path.to_string();
        let file_path = format!("{}{}", self.path, req.url.path);
        let path = Path::new(&file_path);

        if path.is_file() {
            if let Ok(file) = fs::read(path) {
                let res = Response::builder()
                    .body_with_content_type_and_length(path, file)
                    .into();

                return Ok(res);
            }
        }

        if path.is_dir() {
            let links = self.get_filenames(path)?;
            let parent_path = self.get_parent_path(path.parent().unwrap());
            let html_body = self.build_body(req_url, parent_path, links);
            let html_page = self.html_template(html_body).into_bytes();

            let res = Response::builder()
                .header("Content-Type", "text/html; charset=utf-8")
                .header("Content-Length", &html_page.len().to_string())
                .body(html_page)
                .into();

            return Ok(res);
        }

        let html_not_found_page = self
            .html_template("<h1>404 Content Not Found</h1>".to_string())
            .into_bytes();

        let res = Response::builder()
            .status(Status::NotFound)
            .header("Content-Type", "text/html; charset=utf-8")
            .header("Content-Length", &html_not_found_page.len().to_string())
            .body(html_not_found_page)
            .into();

        Ok(res)
    }
}
