use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Methods {
    GET,
    POST,
    PUT,
    DELETE
}

impl fmt::Display for Methods {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Methods::GET => write!(f, "GET"),
            Methods::POST => write!(f, "POST"),
            Methods::PUT => write!(f, "PUT"),
            Methods::DELETE => write!(f, "DELETE")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Methods;

    #[test]
    fn test_format_get() {
        let get_as_string = format!("{}", Methods::GET);
        assert_eq!(String::from("GET"), get_as_string)
    }

    #[test]
    fn test_format_post() {
        let post_as_string = format!("{}", Methods::POST);
        assert_eq!(String::from("POST"), post_as_string)
    }

    #[test]
    fn test_format_put() {
        let put_as_string = format!("{}", Methods::PUT);
        assert_eq!(String::from("PUT"), put_as_string)
    }

    #[test]
    fn test_format_delete() {
        let delete_as_string = format!("{}", Methods::DELETE);
        assert_eq!(String::from("DELETE"), delete_as_string)
    }
}
