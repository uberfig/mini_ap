pub enum HttpMethod {
    Get,
    Post,
}
impl HttpMethod {
    fn stringify(&self) -> &str {
        match self {
            HttpMethod::Get => "get",
            HttpMethod::Post => "post",
        }
    }
}

pub fn signature_string(
    method: HttpMethod,
    path: &str,
    nonce: &str,
    hash: &str,
    // currently versia has made the decision to not include timestamps.
    // I have left this here because I feel that is a design mistake. it
    // can be enabled at any time if they decide to change their decision
    _timestamp: i64,
) -> String {
    format!(
        "{} {} {} {}",
        method.stringify(),
        path,
        nonce,
        hash,
        // timestamp
    )
}
