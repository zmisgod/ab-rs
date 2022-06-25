use reqwest::StatusCode;
use std::error::Error;
use std::fmt;

pub struct Http {
    url: String,
    method: String,
    data: Vec<String>,
    content_type: String,
}

impl Clone for Http {
    fn clone(&self) -> Http {
        Http {
            url: self.url.clone(),
            method: self.method.clone(),
            data: self.data.clone(),
            content_type: self.content_type.clone(),
        }
    }
}

impl fmt::Debug for Http {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "url:{:?} method:{:?} data:{:?} content_type:{:?}",
            self.url, self.method, self.data, self.content_type
        )
    }
}

pub fn build_new_http(url: String) -> Http {
    Http {
        url: url,
        method: String::from("get"),
        data: vec![],
        content_type: String::from("application/json"),
    }
}

impl Http {
    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_content_type(&mut self, content_type: String) {
        self.content_type = content_type;
        self.check_data()
    }

    pub fn set_data(&mut self, data: Vec<String>) {
        self.data = data;
        self.check_data()
    }

    pub fn set_method(&mut self, method: String) {
        self.method = method;
        self.check_data();
    }

    fn check_data(&mut self) {
        if self.method == "get" {
            self.data = vec![];
            self.content_type = String::from("");
        }
    }

    pub async fn send_request(&self) -> Result<StatusCode, Box<dyn Error>> {
        println!("{:?}", self.url);
        let resp = reqwest::get(self.url.clone()).await?;
        let status = resp.status();
        Ok(status)
    }
}
