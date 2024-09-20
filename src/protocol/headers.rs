pub trait Headers {
    fn get(&self, key: &str) -> Option<&str>;
}

pub struct ReqwestHeaders {
    pub headermap: reqwest::header::HeaderMap,
}

impl Headers for ReqwestHeaders {
    fn get(&self, key: &str) -> Option<&str> {
        let val = self.headermap.get(key).map(|x| x.to_str())?;
        match val {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }
}
