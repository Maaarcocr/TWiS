use std::fmt;
use hyper::Result;
use hyper::Error;
use hyper::header::{Header, HeaderFormat};
use hyper::header::parsing::from_comma_delimited;
use regex::Regex;
#[derive(Debug, Clone)]
pub struct Link {
    next: Option<String>,
    prev: Option<String>,
    first: Option<String>,
    last: Option<String>
}

impl Header for Link {
    fn header_name() -> &'static str {
        "Link"
    }

    fn parse_header(raw: &[Vec<u8>]) -> Result<Link> {
        let mut result = Link{first: None, prev: None, last: None, next: None};
        let links: Vec<String> = from_comma_delimited(raw)?;
        let re = Regex::new("<(.*)>; rel=\"(.*)\"").unwrap();
        for link in links {
            let caps = re.captures(link.as_str()).unwrap();
            let url = caps.get(1).unwrap();
            match caps.get(2).unwrap().as_str() {
                "next" => result.next = Some(String::from(url.as_str())),
                "prev" => result.prev = Some(String::from(url.as_str())),
                "first" => result.first = Some(String::from(url.as_str())),
                "last" => result.last = Some(String::from(url.as_str())),
                _ => return Err(Error::Header)
            };
        }
        Ok(result)
    }
}

impl HeaderFormat for Link {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("0")
    }
}