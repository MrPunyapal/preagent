
#[cfg(test)]
mod preagent_v2 {
    use super::*;
    use bytes::BytesMut;

    fn accepts_request(headers: &'static str) -> bool {
        let mut buffer = BytesMut::from(headers);
        let mut method = None;
        Server::parse(
            &mut buffer,
            ParseContext {
                cached_headers: &mut None,
                req_method: &mut method,
                h1_parser_config: Default::default(),
                h1_max_headers: None,
                h1_max_header_size: None,
                preserve_header_case: false,
                #[cfg(feature = "ffi")]
                preserve_header_order: false,
                h09_responses: false,
                #[cfg(feature = "client")]
                on_informational: &mut None,
            },
        )
        .is_ok()
    }

    #[test]
    fn conflicting_content_lengths_must_be_rejected() {
        assert!(!accepts_request(
            "POST / HTTP/1.1\r\nHost: localhost\r\nContent-Length: 1\r\nContent-Length: 2\r\n\r\n"
        ));
        assert!(!accepts_request(
            "POST / HTTP/1.1\r\nHost: localhost\r\nContent-Length: 5\r\nContent-Length: 1\r\n\r\n"
        ));
    }

    #[test]
    fn unambiguous_lengths_are_accepted() {
        assert!(accepts_request(
            "POST / HTTP/1.1\r\nHost: localhost\r\nContent-Length: 4\r\n\r\n"
        ));
        assert!(accepts_request(
            "POST / HTTP/1.1\r\nHost: localhost\r\nContent-Length: 4\r\nContent-Length: 4\r\n\r\n"
        ));
    }
}
