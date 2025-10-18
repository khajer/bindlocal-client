pub struct HttpRequest {}

impl HttpRequest {
    pub fn parse_content_length(headers: &str) -> Option<usize> {
        for line in headers.lines() {
            if line.to_lowercase().starts_with("content-length:") {
                if let Some(value) = line.split(':').nth(1) {
                    if let Ok(length) = value.trim().parse::<usize>() {
                        return Some(length);
                    }
                }
            }
        }
        None
    }
    pub fn parse_content_request_format(headers: &str) -> String {
        let line = headers.lines().nth(0);
        if let Some(value) = line {
            if let Some(space_index) = value.rfind(" ") {
                return value[0..space_index].to_string() + " ";
            }

            return value.to_string();
        }
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_content_length() {
        let headers = "Content-Length: 123\r\n";
        let result = HttpRequest::parse_content_length(headers);
        assert_eq!(result, Some(123));
    }

    #[test]
    fn test_parse_content_length_invalid() {
        let headers = "Content-Length: abc\r\n";
        let result = HttpRequest::parse_content_length(headers);
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_content_length_empty() {
        let headers = "Content-Length:\r\n";
        let result = HttpRequest::parse_content_length(headers);
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_content_request() {
        let headers = "GET / HTTP/1.1\r\n";
        let result = HttpRequest::parse_content_request_format(headers);
        assert_eq!(result, "GET / ");
    }
}
