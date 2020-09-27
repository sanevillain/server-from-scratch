use std::{collections::HashMap, default::Default, io::Error, str::FromStr, string::ToString};

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
        let key: String = key
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect();

        if key.is_empty() || val.is_empty() {
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
        let key = key.trim();

        if key.is_empty() || !self.headers.contains_key(key) {
            return;
        }

        self.headers.remove(key);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let values = self.values(key)?;
        values.first().map(|s| s.to_owned())
    }

    pub fn values(&self, key: &str) -> Option<Vec<String>> {
        let key = key.trim();

        if key.is_empty() || !self.headers.contains_key(key) {
            return None;
        }

        let values = self.headers.get(key).unwrap().clone();
        Some(values)
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
        let mut header = Header::new();

        for line in s.split("\r\n") {
            if line.is_empty() || !line.contains(":") {
                continue;
            }

            let (key, values): (String, String) = (
                line.chars().take_while(|c| *c != ':').collect(),
                line.chars().skip_while(|c| *c != ' ').collect(),
            );

            values.split(",").for_each(|v| {
                header.add(&key, v.trim());
            });
        }

        Ok(header)
    }
}

impl ToString for Header {
    fn to_string(&self) -> String {
        let mut lines = "".to_string();

        for key in self.ordered_keys.clone() {
            let mut line = format!("{}: ", key);

            let values = self.values(&key).unwrap();
            let len = values.len();

            if len == 1 {
                line += values.iter().nth(0).unwrap();
            } else {
                values
                    .iter()
                    .enumerate()
                    .map(|(i, v)| {
                        if i < (len - 1) {
                            format!("{}, ", v)
                        } else {
                            v.to_string()
                        }
                    })
                    .for_each(|v| line += &v);
            }

            lines += &format!("{}\r\n", line);
        }

        lines.to_owned()
    }
}

#[cfg(test)]
mod test_header {
    use super::*;

    #[cfg(test)]
    mod test_add_and_get {
        use super::*;

        #[test]
        fn test_get_non_existing_key_should_return_none() {
            let header = Header::new();
            assert_eq!(None, header.get("Host"));
        }

        #[test]
        fn test_add_empty_strings_shouldnt_be_allowed() {
            let mut header = Header::new();
            header.add("", "");
            header.add("      ", "        ");

            assert_eq!(None, header.get(""));
            assert_eq!(None, header.get("         "));
        }

        #[test]
        fn test_add_special_chars_and_space_should_be_removed() {
            let mut header = Header::new();
            header.add("  \n\t   \r\n\t  ", "  \n\t   \r\n\t  ");
            assert_eq!(None, header.get("  \n\t   \r\n\t  "));
        }

        #[test]
        fn test_add_valid_header_and_value_should_be_retrievable() {
            let mut header = Header::new();

            let (key, val) = ("Host", "docs.apigee.com");
            header.add(key, val);

            assert_eq!(val, header.get(key).unwrap());
        }

        #[test]
        fn test_add_valid_header_with_multiple_values_should_return_first_inserted_value() {
            let mut header = Header::new();

            let (key, vals) = (
                "Accept",
                vec![
                    "text/html",
                    "application/xhtml+xml",
                    "application/xml;q=0.9",
                    "image/webp",
                    "*/*;q=0.8",
                ],
            );

            vals.iter().for_each(|val| header.add(key, val));

            assert_eq!(vals[0], header.get(key).unwrap());
        }
    }

    #[cfg(test)]
    mod test_del {
        use super::*;

        #[test]
        fn test_del_key_should_delete_asociated_value() {
            let mut header = Header::new();

            let (key, val) = ("Host", "docs.apigee.com");
            header.add(key, val);
            header.del(key);

            assert_eq!(None, header.get(key));
        }

        #[test]
        fn test_del_key_should_delete_asociated_values() {
            let mut header = Header::new();

            let (key, vals) = (
                "Accept",
                vec![
                    "text/html",
                    "application/xhtml+xml",
                    "application/xml;q=0.9",
                    "image/webp",
                    "*/*;q=0.8",
                ],
            );

            vals.iter().for_each(|val| header.add(key, val));
            header.del(key);

            assert_eq!(None, header.get(key));
        }
    }

    #[cfg(test)]
    mod test_values {
        use super::*;

        #[test]
        fn test_values_for_non_existing_entry_should_return_none() {
            let header = Header::new();
            assert_eq!(None, header.values("Host"));
        }

        #[test]
        fn test_values_for_existing_key_should_return_vec_with_single_value() {
            let mut header = Header::new();

            let (key, vals) = ("Host", vec!["docs.apigee.com"]);
            vals.iter().for_each(|val| header.add(key, val));

            let expected = vec!["docs.apigee.com"];
            assert_eq!(expected, header.values(key).unwrap());
        }

        #[test]
        fn test_values_for_existing_key_should_return_vec_for_multiple_values() {
            let mut header = Header::new();

            let (key, vals) = (
                "Accept",
                vec![
                    "text/html",
                    "application/xhtml+xml",
                    "application/xml;q=0.9",
                    "image/webp",
                    "*/*;q=0.8",
                ],
            );

            vals.iter().for_each(|val| header.add(key, val));

            assert_eq!(vals, header.values(key).unwrap());
        }
    }

    #[cfg(test)]
    mod test_from_str {
        use super::*;

        #[test]
        fn test_from_str_should_skip_first_line() {
            let header = Header::from_str(
                "GET /api-platform/antipatterns/multi-value-http-headers HTTP/1.1\r\n\\",
            )
            .unwrap();

            assert_eq!(None, header.values("GET"));
        }

        #[test]
        fn test_from_str_should_store_header_key_value() {
            let header = Header::from_str(
                "GET /api-platform/antipatterns/multi-value-http-headers HTTP/1.1\r\n\
                Host: docs.apigee.com\r\n\
                User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:81.0) Gecko/20100101 Firefox/81.0\r\n\\"
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
        fn test_from_str_should_split_header_values_at_the_comas() {
            let header = Header::from_str(
                "GET /api-platform/antipatterns/multi-value-http-headers HTTP/1.1\r\n\
                Host: docs.apigee.com\r\n\
                User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:81.0) Gecko/20100101 Firefox/81.0\r\n\
                Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8\r\n"
            ).unwrap();

            let expected = vec![
                ("Host", vec!["docs.apigee.com"]),
                ("User-Agent", vec!["Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:81.0) Gecko/20100101 Firefox/81.0"]),
                ("Accept", vec!["text/html", "application/xhtml+xml", "application/xml;q=0.9", "image/webp", "*/*;q=0.8"]),
            ];

            for (key, values) in expected.iter() {
                let values: Vec<_> = values.iter().map(|s| s.to_string()).collect();
                assert_eq!(values, header.values(key).unwrap());
            }
        }

        #[test]
        fn test_header_from_str_should_save_all_header_values() {
            let header = Header::from_str(
                "GET /api-platform/antipatterns/multi-value-http-headers HTTP/1.1\r\n\
                Host: docs.apigee.com\r\n\
                User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:81.0) Gecko/20100101 Firefox/81.0\r\n\
                Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8\r\n\
                Accept-Language: en-US,en;q=0.5\r\n\
                Accept-Encoding: gzip, deflate, br\r\n\
                Referer: https://duckduckgo.com/\r\n\
                Upgrade-Insecure-Requests: 1\r\n\
                Connection: keep-alive\r\n\
                Cookie: django_language=en\r\n"
            ).unwrap();

            let expected = vec![
                ("Host", vec!["docs.apigee.com"]),
                ("User-Agent", vec!["Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:81.0) Gecko/20100101 Firefox/81.0"]),
                ("Accept", vec!["text/html", "application/xhtml+xml", "application/xml;q=0.9", "image/webp", "*/*;q=0.8"]),
                ("Accept-Language", vec!["en-US","en;q=0.5"]),
                ("Accept-Encoding", vec!["gzip", "deflate", "br"]),
                ("Referer", vec!["https://duckduckgo.com/"]),
                ("Upgrade-Insecure-Requests", vec!["1"]),
                ("Connection", vec!["keep-alive"]),
                ("Cookie", vec!["django_language=en"]),
            ];

            for (key, values) in expected.iter() {
                let values: Vec<_> = values.iter().map(|s| s.to_string()).collect();
                assert_eq!(values, header.values(key).unwrap());
            }
        }
    }

    #[cfg(test)]
    mod test_to_string {
        use super::*;

        #[test]
        fn test_to_string_with_single_values() {
            let expected = "\
            Host: docs.apigee.com\r\n\
            User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:81.0) Gecko/20100101 Firefox/81.0\r\n";

            let header_string = Header::from_str(expected).unwrap().to_string();
            assert_eq!(expected, header_string);
        }

        #[test]
        fn test_to_string_with_multiple_values() {
            let expected = "\
            Host: docs.apigee.com\r\n\
            User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:81.0) Gecko/20100101 Firefox/81.0\r\n\
            Accept: text/html, application/xhtml+xml, application/xml;q=0.9, image/webp, */*;q=0.8\r\n\
            Accept-Language: en-US, en;q=0.5\r\n\
            Accept-Encoding: gzip, deflate, br\r\n\
            Referer: https://duckduckgo.com/\r\n\
            Upgrade-Insecure-Requests: 1\r\n\
            Connection: keep-alive\r\n\
            Cookie: django_language=en\r\n";

            let header_string = Header::from_str(expected).unwrap().to_string();
            assert_eq!(expected, header_string);
        }
    }
}
