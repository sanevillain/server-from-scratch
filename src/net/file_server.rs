use super::http_request::Request;
use super::http_server::Handler;
use std::{fs, io, path::Path, str};

const TEMPLATE: &str = "\
HTTP/1.1 200 OK\r\n\
Content-Type: text/html; charset=UTF-8\r\n\r\n\
<!DOCTYPE html>\
<html>\
    <head>\
        <title>Hello</title>\
    </head>\
    <body>\
        {{ }}
    </body>\
</html>\r\n\r\n";

#[derive(Clone)]
pub struct FileServer {
    path: &'static str,
}

impl FileServer {
    pub fn new(path: &'static str) -> Self {
        Self { path }
    }

    fn build_page(path: String, filenames: Vec<String>) -> String {
        let mut page: Vec<String> = vec![];

        let ul_list = FileServer::build_ul_list(path, filenames);
        let halves: Vec<_> = TEMPLATE.split("{{ }}").collect();

        halves.iter().enumerate().for_each(|(i, half)| {
            if i == (halves.len() - 1) {
                page.extend(ul_list.clone().into_iter());
            }

            page.push(half.to_string());
        });

        page.join("")
    }

    fn build_ul_list(path: String, filenames: Vec<String>) -> Vec<String> {
        let mut list = vec![];
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
