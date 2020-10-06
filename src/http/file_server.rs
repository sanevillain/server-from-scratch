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

    fn build_page(&self, path: String, previous_path: String, filenames: Vec<String>) -> String {
        const FIRST_HALF: &str = "<!DOCTYPE html><html><head><title>Hello</title></head><body>";
        const SECOND_HALF: &str = "</body></html>\r\n\r\n";

        let mut page: Vec<String> = vec![];

        page.push(FIRST_HALF.to_string());
        page.extend(self.build_body(path, previous_path, filenames));
        page.push(SECOND_HALF.to_string());
        page.join("")
    }

    fn build_body(
        &self,
        path: String,
        previous_path: String,
        filenames: Vec<String>,
    ) -> Vec<String> {
        let mut list = vec![];

        println!("FileNames: {:?} {}", filenames, path);

        list.push(format!("<h3>{}</h3>", path));
        list.push(format!(
            "<a href=\"{}\">Back</a>",
            if previous_path == "" {
                "/".to_string()
            } else {
                previous_path
            }
        ));
        list.push(String::from("<ul>"));
        list.extend(filenames.iter().map(|filename| {
            format!(
                "<li><a href=\"{}{}\">{}</a></li>",
                if path != "/" {
                    path.to_string()
                } else {
                    "".to_string()
                },
                filename,
                filename
            )
        }));
        list.push(String::from("</ul>"));
        list
    }
}

impl Handler for FileServer {
    fn serve_http(&self, req: Request) -> io::Result<Response> {
        let p = format!("{}{}", self.path, req.url.path);
        let path = Path::new(&p);

        if path.is_file() {
            if let Ok(file) = fs::read(path) {
                return Ok(Response::builder()
                    .header("Content-Type", "text")
                    .body(file)
                    .into());
            }
        }

        if !path.is_dir() {
            let not_found = "<!DOCTYPE html><html><head><title>Hello</title></head><body><h1>404 Content Not Found</body></html>\r\n\r\n";

            return Ok(Response::builder()
                .header("Content-Type", "text/html")
                .body(String::from(not_found).into_bytes())
                .status(Status::NotFound)
                .into());
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
            .map(|filename| format!("/{}", filename))
            .collect::<Vec<String>>();

        let mut previous_path = path.parent().unwrap().to_str().unwrap().to_string();

        if previous_path.starts_with(self.path) {
            previous_path = previous_path
                .strip_prefix(self.path)
                .expect("Couldn't trim previous path")
                .to_string();
        } else {
            previous_path = "/".to_string();
        }

        let body = self
            .build_page(req.url.path.to_string(), previous_path, links)
            .into_bytes();

        Ok(Response::builder().body(body).into())
    }
}
