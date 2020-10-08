use super::{method::Method, url::URL, version::Version};
use std::{
    collections::HashMap,
    default::Default,
    io::{Error, ErrorKind},
    str::FromStr,
    string::ToString,
};

#[derive(Debug)]
pub struct Header {
    headers: HashMap<String, Vec<String>>,
    ordered_keys: Vec<String>,
}

impl Header {
    pub fn new() -> Self {
        Self {
            headers: HashMap::<String, Vec<String>>::new(),
            ordered_keys: Vec::<String>::new(),
        }
    }

    pub fn add(&mut self, key: &str, val: &str) {
        let original_key = key.to_string();
        let key = key
            .trim()
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>();

        if key.is_empty() || key.len() != original_key.len() || val.is_empty() {
            return;
        }

        if !self.headers.contains_key(&key) {
            self.headers.insert(key.to_string(), vec![]);
            self.ordered_keys.push(key.to_string());
        }

        let values = self.headers.get_mut(&key).unwrap();
        values.push(val.to_string());
    }

    pub fn del(&mut self, key: &str) {
        if self.headers.contains_key(key) {
            self.headers.remove(key);
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.values(key)?.first().map(|s| s.to_owned())
    }

    pub fn values(&self, key: &str) -> Option<Vec<String>> {
        if self.headers.contains_key(key) {
            Some(self.headers.get(key).unwrap().clone())
        } else {
            None
        }
    }
}

impl Header {
    pub fn from_lines(lines: Vec<&str>) -> Result<Header, Error> {
        let mut header = Header::new();

        for line in lines.iter() {
            let line = line.trim();

            if !line.contains(":") || !line.contains(" ") {
                return Err(Error::new(ErrorKind::InvalidInput, "Invalid headers line!"));
            }

            let key = line.chars().take_while(|c| *c != ':').collect::<String>();
            let values = line.chars().skip_while(|c| *c != ' ').collect::<String>();

            if key.is_empty() || values.is_empty() {
                return Err(Error::new(ErrorKind::InvalidInput, "Invalid headers line!"));
            }

            values
                .split(",")
                .for_each(|val| header.add(&key, val.trim()));
        }

        Ok(header)
    }
}

impl Default for Header {
    fn default() -> Self {
        Header::new()
    }
}

impl FromStr for Header {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = Err(Error::new(
            ErrorKind::InvalidInput,
            "Couldn't parse headers!",
        ));

        if s.is_empty() || !s.contains("\r\n\r\n") {
            return err;
        }

        let headers_body = s.split("\r\n\r\n").collect::<Vec<&str>>();
        let headers = match headers_body.first() {
            Some(headers) => headers,
            _ => return err,
        };

        let lines = headers
            .split("\r\n")
            .filter_map(|l| {
                if !l.trim().is_empty() {
                    Some(l.trim())
                } else {
                    None
                }
            })
            .collect::<Vec<&str>>();

        if lines.is_empty() {
            return err;
        }

        let first_line = lines.first().unwrap().split(" ").collect::<Vec<&str>>();
        if first_line.len() < 3 {
            return err;
        }

        if let Err(e) = Method::from_str(first_line[0]) {
            return Err(e);
        } else if let Err(e) = URL::from_str(first_line[1]) {
            return Err(e);
        } else if let Err(e) = Version::from_str(first_line[2]) {
            return Err(e);
        }

        let lines = lines.iter().skip(1).map(|s| *s).collect::<Vec<&str>>();
        Header::from_lines(lines)
    }
}

impl ToString for Header {
    fn to_string(&self) -> String {
        let build_key_values_line = |key: &str| -> String {
            let values = self.values(key).unwrap();
            let values = values
                .iter()
                .enumerate()
                .map(|(i, val)| {
                    if i < values.len() - 1 {
                        format!("{}, ", val)
                    } else {
                        format!("{}\r\n", val)
                    }
                })
                .collect::<Vec<String>>();

            format!("{}: {}", key, values.join(""))
        };

        let lines = self
            .ordered_keys
            .iter()
            .map(|key| build_key_values_line(key))
            .collect::<Vec<String>>();

        format!("{}\r\n", lines.join(""))
    }
}

#[cfg(test)]
mod test_method_add_and_get {
    use super::*;

    #[test]
    fn test_get_non_existing_key_should_return_none() {
        let header = Header::new();
        assert_eq!(None, header.get("Host"));
    }

    #[test]
    fn test_add_should_only_add_alphanumeric_keys() {
        let mut header = Header::new();
        let key_val_expected = vec![
            ("", "", None),
            (" ", " ", None),
            ("\n\t\r\n\t", "\n\t\r\n\t", None),
            ("&*@@$^^!", "invalid", None),
            ("still&*@@$invalid^^!", "invalid", None),
            ("valid", "valid", Some(String::from("valid"))),
            (
                "Host",
                "docs.apigee.com",
                Some(String::from("docs.apigee.com")),
            ),
        ];

        key_val_expected
            .iter()
            .for_each(|(key, val, _)| header.add(key, val));
        key_val_expected
            .iter()
            .for_each(|(key, _, expected)| assert_eq!(*expected, header.get(key)));
    }

    #[test]
    fn test_get_with_multiple_values_should_return_first() {
        let mut header = Header::new();
        let (key, vals, expected) = (
            "Accept",
            vec![
                "text/html",
                "application/xhtml+xml",
                "application/xml;q=0.9",
                "image/webp",
                "*/*;q=0.8",
            ],
            String::from("text/html"),
        );

        vals.iter().for_each(|val| header.add(key, val));
        assert_eq!(expected, header.get(key).unwrap());
    }
}

#[cfg(test)]
mod test_method_del {
    use super::*;

    #[test]
    fn test_del_key_should_delete_asociated_value() {
        let mut header = Header::new();
        let (key, val, expected) = ("Host", "docs.apigee.com", None);
        header.add(key, val);
        header.del(key);
        assert_eq!(expected, header.get(key));
    }

    #[test]
    fn test_del_key_should_delete_all_asociated_values() {
        let mut header = Header::new();
        let (key, vals, expected) = (
            "Accept",
            vec![
                "text/html",
                "application/xhtml+xml",
                "application/xml;q=0.9",
                "image/webp",
                "*/*;q=0.8",
            ],
            None,
        );

        vals.iter().for_each(|val| header.add(key, val));
        header.del(key);
        assert_eq!(expected, header.get(key));
    }
}

#[cfg(test)]
mod test_method_values {
    use super::*;

    #[test]
    fn test_values_for_non_existing_entry_should_return_none() {
        let header = Header::new();
        let (key, expected) = ("Host", None);
        assert_eq!(expected, header.values(key));
    }

    #[test]
    fn test_values_for_existing_key_should_return_vec_with_single_value() {
        let mut header = Header::new();
        let (key, vals, expected) = (
            "Host",
            vec!["docs.apigee.com"],
            Some(vec![String::from("docs.apigee.com")]),
        );

        vals.iter().for_each(|val| header.add(key, val));
        assert_eq!(expected, header.values(key));
    }

    #[test]
    fn test_values_for_existing_key_should_return_vec_for_multiple_values() {
        let mut header = Header::new();
        let (key, vals, expected) = (
            "Accept",
            vec![
                "text/html",
                "application/xhtml+xml",
                "application/xml;q=0.9",
                "image/webp",
                "*/*;q=0.8",
            ],
            Some(vec![
                String::from("text/html"),
                String::from("application/xhtml+xml"),
                String::from("application/xml;q=0.9"),
                String::from("image/webp"),
                String::from("*/*;q=0.8"),
            ]),
        );

        vals.iter().for_each(|val| header.add(key, val));
        assert_eq!(expected, header.values(key));
    }
}

#[cfg(test)]
mod test_from_str {
    use super::*;

    #[test]
    fn test_from_str_should_return_io_error_on_invalid_input() {
        let vals = vec![
            "",
            "     ",
            "1232392asdljas",
            "\r\n\r\n",
            "     \r\n\r\n",
            "     \r\n\r\nwhatever",
            "1232392asdljas\r\n\r\n",
            "1232392asdljas\r\n\r\nwhatever",
        ];
        let headers = vals.iter().map(|v| Header::from_str(v));
        let expected_err = Error::new(ErrorKind::InvalidInput, "Couldn't parse headers!");

        headers.for_each(|h| match h {
            Ok(_) => panic!("Header shouldn't be retrievable!"),
            Err(e) => {
                assert_eq!(e.kind(), expected_err.kind());
                assert_eq!(e.to_string(), expected_err.to_string());
            }
        });
    }

    #[test]
    fn test_from_str_should_return_io_error_on_invalid_method() {
        let header = Header::from_str("UNKNOWN / HTTP/1.1\r\n\r\n");
        let expected_err = Error::new(ErrorKind::InvalidInput, "Invalid HTTP method!");

        match header {
            Ok(_) => panic!("Header shouldn't be retrievable!"),
            Err(e) => {
                assert_eq!(e.kind(), expected_err.kind());
                assert_eq!(e.to_string(), expected_err.to_string());
            }
        };
    }

    #[test]
    fn test_from_str_should_return_io_error_on_invalid_url() {
        let header = Header::from_str("GET invalid/ HTTP/1.1\r\n\r\n");
        let expected_err = Error::new(ErrorKind::InvalidInput, "Invalid url!");

        match header {
            Ok(_) => panic!("Header shouldn't be retrievable!"),
            Err(e) => {
                assert_eq!(e.kind(), expected_err.kind());
                assert_eq!(e.to_string(), expected_err.to_string());
            }
        };
    }

    #[test]
    fn test_from_str_should_return_io_error_on_unsupported_http_version() {
        let vals = vec![
            "GET / HTTP/1.0\r\n\r\n",
            "POST /valid HTTP/2\r\n\r\n",
            "PUT /valid HTTP/3\r\n\r\n",
        ];
        let headers = vals.iter().map(|v| Header::from_str(v));
        let expected_err = Error::new(ErrorKind::InvalidInput, "HTTP version not supported!");

        headers.for_each(|h| match h {
            Ok(_) => panic!("Header shouldn't be retrievable!"),
            Err(e) => {
                assert_eq!(e.kind(), expected_err.kind());
                assert_eq!(e.to_string(), expected_err.to_string());
            }
        });
    }

    #[test]
    fn test_from_str_should_skip_first_line() {
        let string = "GET /api-platform/antipatterns/multi-value-http-headers HTTP/1.1\r\n\r\n";
        let header = Header::from_str(string).unwrap();

        assert_eq!(None, header.values("GET"));
    }

    #[test]
    fn test_from_str_should_store_header_key_value() {
        let header = Header::from_str(
                "GET /api-platform/antipatterns/multi-value-http-headers HTTP/1.1\r\n\
                Host: docs.apigee.com\r\n\
                User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:81.0) Gecko/20100101 Firefox/81.0\r\n\r\n"
            ).unwrap();

        let expected = vec![
                ("Host", vec!["docs.apigee.com"]),
                ("User-Agent", vec!["Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:81.0) Gecko/20100101 Firefox/81.0"]),
            ];

        for (key, values) in expected.iter() {
            let values: Vec<_> = values.iter().map(|s| s.to_string()).collect();
            assert_eq!(values, header.values(key).unwrap());
        }
    }

    #[test]
    fn test_from_str_should_split_add_all_header_values() {
        let header = Header::from_str(
                "GET /api-platform/antipatterns/multi-value-http-headers HTTP/1.1\r\n\
                Host: docs.apigee.com\r\n\
                User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:81.0) Gecko/20100101 Firefox/81.0\r\n\
                Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8\r\n\r\n"
            ).unwrap();

        let expected = vec![
                ("Host", Some(vec![String::from("docs.apigee.com")])),
                ("User-Agent", Some(vec![String::from("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:81.0) Gecko/20100101 Firefox/81.0")])),
                ("Accept", Some(vec![
                    String::from("text/html"),
                    String::from("application/xhtml+xml"),
                    String::from("application/xml;q=0.9"),
                    String::from("image/webp"),
                    String::from("*/*;q=0.8")
                ])),
            ];

        expected
            .iter()
            .for_each(|(key, values)| assert_eq!(*values, header.values(key)));
    }
}

#[cfg(test)]
mod test_to_string {
    use super::*;

    #[test]
    fn test_to_string_should_return_io_error_on_invalid_line() {
        let header = Header::from_str(
            "GET /api-platform/antipatterns/multi-value-http-headers HTTP/1.1\r\n\
            Host docs.apigee.com\r\n\
            User-Agent Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:81.0) Gecko/20100101 Firefox/81.0\r\n\
            Accept > text/html, application/xhtml+xml, application/xml;q=0.9, image/webp, */*;q=0.8\r\n\
            Accept-Language< en-US, en;q=0.5\r\n\
            Accept-Encoding: gzip, deflate, br\r\n\
            Referer: https://duckduckgo.com/\r\n\
            Upgrade-Insecure-Requests: 1\r\n\
            Connection: keep-alive\r\n\
            Cookie: django_language=en\r\n\r\n"
        );
        let expected_err = Error::new(ErrorKind::InvalidInput, "Invalid headers line!");

        match header {
            Ok(_) => panic!("Header shouldn't be retrievable!"),
            Err(e) => {
                assert_eq!(e.kind(), expected_err.kind());
                assert_eq!(e.to_string(), expected_err.to_string());
            }
        };
    }

    #[test]
    fn test_to_string_with_multiple_values() {
        let header = Header::from_str(
            "GET /api-platform/antipatterns/multi-value-http-headers HTTP/1.1\r\n\
            Host: docs.apigee.com\r\n\
            User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:81.0) Gecko/20100101 Firefox/81.0\r\n\
            Accept: text/html, application/xhtml+xml, application/xml;q=0.9, image/webp, */*;q=0.8\r\n\
            Accept-Language: en-US, en;q=0.5\r\n\
            Accept-Encoding: gzip, deflate, br\r\n\
            Referer: https://duckduckgo.com/\r\n\
            Upgrade-Insecure-Requests: 1\r\n\
            Connection: keep-alive\r\n\
            Cookie: django_language=en\r\n\r\n"
        ).unwrap();

        let expected = "\
            Host: docs.apigee.com\r\n\
            User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:81.0) Gecko/20100101 Firefox/81.0\r\n\
            Accept: text/html, application/xhtml+xml, application/xml;q=0.9, image/webp, */*;q=0.8\r\n\
            Accept-Language: en-US, en;q=0.5\r\n\
            Accept-Encoding: gzip, deflate, br\r\n\
            Referer: https://duckduckgo.com/\r\n\
            Upgrade-Insecure-Requests: 1\r\n\
            Connection: keep-alive\r\n\
            Cookie: django_language=en\r\n\r\n";

        let header_string = header.to_string();
        assert_eq!(expected, header_string);
    }
}
