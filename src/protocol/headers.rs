pub trait Headers {
    fn get(&self, key: &str) -> Option<String>;
}

pub struct ReqwestHeaders {
    pub headermap: reqwest::header::HeaderMap,
}

impl Headers for ReqwestHeaders {
    fn get(&self, key: &str) -> Option<String> {
        let val = self.headermap.get(key).map(|x| x.to_str())?;
        match val {
            Ok(x) => Some(x.to_string()),
            Err(_) => None,
        }
    }
}

pub struct ActixHeaders {
    pub headermap: actix_web::http::header::HeaderMap,
}

impl Headers for ActixHeaders {
    fn get(&self, key: &str) -> Option<String> {
        let val = self.headermap.get(key)?;
        let val = String::from_utf8(val.as_bytes().to_vec());
        match val {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }
}