
#[cfg(test)]
mod preagent_v2 {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn shared_fast_header_scan_does_not_suppress_complete_requests() {
        let mut raw = BytesMut::from("GET /ok HTTP/1.1\r\nHost: localhost\r\n\r\n");
        let mut method = None;
        let result = parse_headers::<Server>(&mut raw, Some(1), ParseContext {
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
        });
        assert!(result.unwrap().is_some(), "complete headers must parse");
    }

    #[test]
    fn shared_fast_header_scan_does_not_suppress_complete_responses() {
        let mut raw = BytesMut::from("HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n");
        let result = parse_headers::<Client>(&mut raw, Some(1), ParseContext {
            cached_headers: &mut None,
            req_method: &mut Some(crate::Method::GET),
            h1_parser_config: Default::default(),
            h1_max_headers: None,
            h1_max_header_size: None,
            preserve_header_case: false,
            #[cfg(feature = "ffi")]
            preserve_header_order: false,
            h09_responses: false,
            #[cfg(feature = "client")]
            on_informational: &mut None,
        });
        assert!(result.unwrap().is_some(), "complete response headers must parse");
    }

    #[test]
    fn malformed_complete_headers_are_not_treated_as_incomplete() {
        let mut raw = BytesMut::from("GET /bad HTTP/1.1\r\nBad Header: value\r\n\r\n");
        let mut method = None;
        let result = parse_headers::<Server>(&mut raw, Some(1), ParseContext {
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
        });
        assert!(result.is_err(), "invalid complete headers must produce an error");
    }
}
