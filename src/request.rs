#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    is_secured: bool,
    hostname: String
}

fn is_https(url: &str) -> bool {
    url.contains("https://")
}

fn is_http(url: &str) -> bool {
    url.contains("http://")
}

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

impl Request {
    pub fn new(url: &str) -> Self {
        let is_secured;
        if is_https(url) {
            is_secured = true;
        } else if is_http(url) {
            is_secured = false;
        } else {
            panic!("Invalid protocol in served url")
        }

        let hostname = parse_hostname(url);

        Self {
            is_secured,
            hostname
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECURED_URL: &str = "https://xyz.com";
    const INSECURED_URL: &str = "http://xyz.com";
    const FTP_URL: &str = "ftp://xyz.com";

    // <-- [ISSUE 1]
    // The two function below got created because of problem with initializing hostname as string
    // This will be fixed in future, when I will find solution
    fn init_secured_request() -> Request {
        let secured_request = Request { is_secured: true, hostname: String::from("xyz.com") };
        secured_request
    }

    fn init_insecured_request() -> Request {
        let insecured_request = Request { is_secured: false, hostname: String::from("xyz.com") };
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
    fn test_construct_https_request() {
        let req = Request::new(SECURED_URL);
        assert_eq!(req, init_secured_request())
    }

    #[test]
    fn test_construct_http_request() {
        let req = Request::new(INSECURED_URL);
        assert_eq!(req, init_insecured_request())
    }

    #[test]
    #[should_panic]
    fn test_construct_ftp_request() {
        let _req = Request::new(FTP_URL);
    }
}
