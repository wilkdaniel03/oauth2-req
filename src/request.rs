use std::fmt;
use std::net::TcpStream;
use std::io::{Write, Read};
use std::error::Error;
use native_tls::TlsConnector;
use crate::methods::Methods;

#[derive(Debug, Clone, PartialEq)]
pub struct Request<'a> {
    method: Methods,
    is_secured: bool,
    hostname: String,
    path: Option<String>,
    token: &'a str 
}

impl fmt::Display for Request<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path = self.path.clone().unwrap();

        write!(f, "{} {} HTTP/1.1\r\nAuthorization: Bearer {}\r\nHost: {}\r\nConnection: Close\r\n\r\n",
            self.method,
            path,
            self.token,
            self.hostname
        )
    }
}

fn is_https(url: &str) -> bool {
    url.contains("https://")
}

fn is_http(url: &str) -> bool {
    url.contains("http://")
}

// <-- [ISSUE 2] 
// parse_hostname and parse_path are iterating exactly the same url string twice
fn parse_hostname(url: &str) -> String {
    let mut slash_counter = 0;
    let mut hostname = String::from("");
    for char in url.chars() {
        if char == '/' {
            if slash_counter < 2 {
                slash_counter += 1;
                continue;
            } else {
                break;
            }
        }

        if slash_counter == 2 {
            hostname.push(char);
        }
    }

    hostname
}

fn parse_path(url: &str) -> Option<String> {
    let mut slash_counter = 0;
    let mut path = String::from("");
    for char in url.chars() {
        if char == '/' {
            if slash_counter < 3 {
                slash_counter += 1;
                continue;
            }
        }

        if slash_counter == 3 {
            path.push(char);
        }
    }

    if path.is_empty() {
        return None;
    }

    Some(path)
}
// -->

impl <'a> Request<'a> {
    pub fn new(method: Methods, url: &str, token: &'a str) -> Self {
        let is_secured;
        if is_https(url) {
            is_secured = true;
        } else if is_http(url) {
            is_secured = false;
        } else {
            panic!("Invalid protocol in served url")
        }

        let hostname = parse_hostname(url);
        let path = match parse_path(url) {
            Some(p) => Some(format!("/{}", p)),
            None => None
        };

        Self {
            method,
            is_secured,
            hostname,
            path,
            token
        }
    }

    pub fn send(&self) -> Result<Vec<u8>, &str> {
        let mut res = vec![];
        let _ = self.send_as_insecured(&mut res).unwrap();

        Ok(res)
    }

    fn send_as_insecured(&self, buf: &mut Vec<u8>) -> Result<(), &str> {
        let addr = format!("{}:80", self.hostname);
        let addr = addr.as_str();

        let request = format!("{}", self);
        let request = request.as_bytes();
        
        let mut stream = TcpStream::connect(addr).unwrap();
        stream.write_all(request).unwrap();
        stream.read_to_end(buf).unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECURED_URL: &str = "https://xyz.com";
    const SECURED_URL_WITH_PATH: &str = "https://xyz.com/api/v1/hello";
    const INSECURED_URL: &str = "http://xyz.com";
    const FTP_URL: &str = "ftp://xyz.com";
    const TOKEN: &str = "secret";

    // <-- [ISSUE 1]
    // The two function below got created because of problem with initializing hostname as string
    // This will be fixed in future, when I will find solution
    fn init_secured_request<'a>() -> Request<'a> {
        let secured_request = Request { method: Methods::GET, is_secured: true, hostname: String::from("xyz.com"), path: None, token: "secret" };
        secured_request
    }

    fn init_insecured_request<'a>() -> Request<'a> {
        let insecured_request = Request { method: Methods::GET, is_secured: false, hostname: String::from("xyz.com"), path: None, token: "secret" };
        insecured_request
    }
    // -->

    #[test]
    fn test_is_https_returns_true() {
        assert_eq!(is_https(&SECURED_URL), true)
    }

    #[test]
    fn test_is_https_returns_false() {
        assert_eq!(is_https(&INSECURED_URL), false)
    }

    #[test]
    fn test_is_http_returns_true() {
        assert_eq!(is_http(&INSECURED_URL), true)
    }

    #[test]
    fn test_is_http_returns_false() {
        assert_eq!(is_http(&SECURED_URL), false)
    }

    #[test]
    fn test_parse_hostname() {
        let hostname = parse_hostname(SECURED_URL);
        assert_eq!(hostname, String::from("xyz.com"))
    }

    #[test]
    fn test_parse_path() {
        let path = parse_path(SECURED_URL_WITH_PATH);
        assert_eq!(path, Some(String::from("api/v1/hello")))
    }

    #[test]
    fn test_parse_path_without_path() {
        let path = parse_path(SECURED_URL);
        assert_eq!(path, None)
    }

    #[test]
    fn test_construct_https_request() {
        let req = Request::new(Methods::GET, SECURED_URL, TOKEN);
        assert_eq!(req, init_secured_request())
    }

    #[test]
    fn test_construct_http_request() {
        let req = Request::new(Methods::GET, INSECURED_URL, TOKEN);
        assert_eq!(req, init_insecured_request())
    }

    #[test]
    #[should_panic]
    fn test_construct_ftp_request() {
        let _req = Request::new(Methods::GET, FTP_URL, TOKEN);
    }
}
