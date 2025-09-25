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
}
