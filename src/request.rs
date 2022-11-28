pub struct Request {
    is_secured: bool
}

fn is_https(url: &str) -> bool {
    url.contains("https://")
}

fn is_http(url: &str) -> bool {
    url.contains("http://")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECURED_URL: &str = "https://xyz.com";
    const INSECURED_URL: &str = "http://xyz.com";

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
}
