#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    is_secured: bool
}

fn is_https(url: &str) -> bool {
    url.contains("https://")
}

fn is_http(url: &str) -> bool {
    url.contains("http://")
}

impl Request {
    pub fn new(url: &str) -> Self {
        if is_https(url) {
            Self { is_secured: true }
        } else if is_http(url) {
            Self { is_secured: false }
        } else {
            panic!("Invalid protocol in served url")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECURED_URL: &str = "https://xyz.com";
    const INSECURED_URL: &str = "http://xyz.com";
    const FTP_URL: &str = "ftp://xyz.com";

    const SECURED_REQUEST: Request = Request { is_secured: true };
    const INSECURED_REQUEST: Request = Request { is_secured: false };

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
    fn test_construct_https_request() {
        let req = Request::new(SECURED_URL);
        assert_eq!(req, SECURED_REQUEST)
    }

    #[test]
    fn test_construct_http_request() {
        let req = Request::new(INSECURED_URL);
        assert_eq!(req, INSECURED_REQUEST)
    }

    #[test]
    #[should_panic]
    fn test_construct_ftp_request() {
        let _req = Request::new(FTP_URL);
    }
}
