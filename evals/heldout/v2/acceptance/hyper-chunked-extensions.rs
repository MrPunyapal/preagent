
#[cfg(test)]
mod preagent_v2 {
    use super::*;

    #[tokio::test]
    async fn chunk_extensions_keep_the_correct_body_bytes() {
        let mut raw = Bytes::from_static(b"3;note=ok\r\nabc\r\n0\r\n\r\n");
        let mut decoder = Decoder::chunked(None, None);
        let frame = decoder
            .decode_fut(&mut raw)
            .await
            .expect("valid chunk extension must not be rejected");
        assert_eq!(frame.into_data().expect("data frame").as_ref(), b"abc");
    }

    #[tokio::test]
    async fn ordinary_chunks_still_decode_without_extensions() {
        let mut raw = Bytes::from_static(b"3\r\nabc\r\n0\r\n\r\n");
        let mut decoder = Decoder::chunked(None, None);
        let frame = decoder.decode_fut(&mut raw).await.unwrap();
        assert_eq!(frame.into_data().unwrap().as_ref(), b"abc");
    }
}
