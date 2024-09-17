use serde::Deserialize;

enum HttpMethod {
    Get,
    Post
}
impl HttpMethod {
    fn stringify(&self) -> &str {
        match self {
            HttpMethod::Get => "get",
            HttpMethod::Post => "post",
        }
    }
}

fn signature_string(method: HttpMethod, path: &str, nonce: &str, hash: &str) -> String {
    format!("{} {} {} {}", method.stringify(), path, nonce, hash)
}

pub async fn versia_fetch<T: for<'a> Deserialize<'a>>() -> T {
    todo!()
}