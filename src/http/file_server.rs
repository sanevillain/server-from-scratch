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

        list.push(format!("<h3>{}</h3>", path));
        list.push(format!("<a href=\"{}\">Back</a>", previous_path));
        list.extend(self.build_ul(path, filenames));

        list
    }

    fn build_ul(&self, path: String, filenames: Vec<String>) -> Vec<String> {
        let mut list = vec![];

        list.push("<ul>".to_string());
        filenames.iter().for_each(|filename| {
            let li = format!(
                "<li><a href=\"{}{}\">{}</a></li>",
                if path != "/" {
                    path.to_string()
                } else {
                    "".to_string()
                },
                filename,
                filename
            );

            list.push(li);
        });
        list.push("</ul>".to_string());

        list
    }
}

impl FileServer {
    fn get_parent_path(&self, parenth_path: &Path) -> String {
        let parent_path_str = parenth_path.to_str().unwrap_or("").to_string();

        if parenth_path.starts_with(self.path) {
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
                let content_type = mime_guess::from_path(path)
                    .first_raw()
                    .unwrap_or("text/plain");

                let res = Response::builder()
                    .header("Content-Type", content_type)
                    .body(file)
                    .into();

                return Ok(res);
            }
        } else if path.is_dir() {
            let links = self.get_filenames(path)?;
            let parent_path = self.get_parent_path(path.parent().unwrap());
            let body = self.build_page(req_url, parent_path, links).into_bytes();
            let res = Response::builder().body(body).into();

            return Ok(res);
        }

        let not_found = "<!DOCTYPE html><html><head><title>Hello</title></head><body><h1>404 Content Not Found</body></html>\r\n\r\n";
        let res = Response::builder()
            .header("Content-Type", "text/html")
            .status(Status::NotFound)
            .body(not_found.to_string().into_bytes())
            .into();

        Ok(res)
    }
}
