use reqwest::StatusCode;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::time::Duration;
use http::header::HeaderName;


pub struct Http {
    url: String,
    method: String,
    data: Vec<String>,
    content_type: String,
    debug: bool,
    timeout: i32,
    headers: HashMap<String, String>,
}

impl Clone for Http {
    fn clone(&self) -> Http {
        Http {
            url: self.url.clone(),
            method: self.method.clone(),
            data: self.data.clone(),
            content_type: self.content_type.clone(),
            debug: self.debug,
            timeout: self.timeout,
            headers: self.headers.clone(),
        }
    }
}

impl fmt::Debug for Http {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut content_type = String::from("form-data");
        if content_type != "json" {
            content_type = String::from("form-data");
        }
        let mut method = self.method.clone();
        if self.method != "post" {
            method = String::from("get");
        }
        write!(
            f,
            "url:{:?} method:{:?} data:{:?} content_type:{:?} headers:{:?}
            timeout:{:?} debug:{:?}",
            self.url, method, self.data, content_type, self.headers,
            self.timeout, self.debug
        )
    }
}

pub fn build_new_http(url: String) -> Http {
    Http {
        url,
        method: String::from(""),
        data: vec![],
        content_type: String::from(""),
        debug: false,
        timeout: 0,
        headers: HashMap::new(),
    }
}

impl Http {
    pub fn set_headers(&mut self, headers:Vec<(String, String)>) {
        for (key, value) in headers.iter() {
            self.headers.insert(key.to_string(), value.to_string());
        }
    }

    pub fn set_content_type(&mut self, content_type: String) {
        if content_type == "" {
            self.content_type = String::from("form-data");
        }else{
            self.content_type = content_type;
        }
    }

    pub fn set_timeout(&mut self, timeout: i32) {
        self.timeout = timeout;
    }

    pub fn set_debug(&mut self, show: bool) {
        self.debug = show
    }

    pub fn set_data(&mut self, data: Vec<String>) {
        self.data = data;
    }

    pub fn set_method(&mut self, method: String) {
        self.method = method;
    }

    fn check_data(&mut self) {
        self.parse_data_to_form();
        if self.method == "get" {
            self.data = vec![];
            self.content_type = String::from("");
        }
    }

    pub async fn send_request(&mut self) -> Result<StatusCode, Box<dyn Error>> {
        self.check_data();
        if self.method == "post" {
            return self.send_post().await;
        } else {
            return self.send_get().await;
        }
    }

    async fn send_get(&self) -> Result<StatusCode, Box<dyn Error>> {
        let resp = self.builder_client()?.get(self.url.clone()).send().await?;
        let status = resp.status();
        if self.debug {
            println!("{:?}", resp.text().await?);
        }
        Ok(status)
    }

    fn parse_data_to_form(&self) -> Vec<(String, String)> {
        let mut data_vec = vec![];
        for (_, val) in self.data.iter().enumerate() {
            let mut one: (String, String) = (String::from(""), String::from(""));
            let exp: Vec<&str> = val.split("=").collect();
            if exp.len() > 1 {
                match exp.get(0) {
                    Some(i) => {
                        one.0 = i.to_string();
                    }
                    _ => {}
                }
                match exp.get(1) {
                    Some(i) => {
                        one.1 = i.to_string();
                    }
                    _ => {}
                }
                data_vec.push(one);
            }
        }
        data_vec
    }

    fn parse_data_to_json(&self) -> HashMap<String, String> {
        let mut hash_vec = HashMap::new();
        for (_, val) in self.data.iter().enumerate() {
            let exp: Vec<&str> = val.split("=").collect();
            let mut k = String::from("");
            let mut v = String::from("");
            if exp.len() > 1 {
                match exp.get(0) {
                    Some(i) => {
                        k = i.to_string();
                    }
                    _ => {}
                }
                match exp.get(1) {
                    Some(i) => {
                        v = i.to_string();
                    }
                    _ => {}
                }
                hash_vec.insert(k, v);
            }
        }
        hash_vec
    }

    pub fn builder_client(&self) -> reqwest::Result<reqwest::Client> {
        let mut client = reqwest::Client::builder();
        if self.timeout > 0 {
            client = client.timeout(Duration::from_millis(self.timeout as u64))
        }
        if self.headers.len() > 0 {
            let mut header_map = HeaderMap::new();
            for (key, value) in self.headers.iter() {
                header_map.insert(HeaderName::from_lowercase(key.as_bytes()).unwrap(), value.parse().unwrap());
            }
            client = client.default_headers(header_map);
        }
        client.build()
    }

    pub async fn get_response(
        &self,
        client: reqwest::RequestBuilder,
    ) -> reqwest::Result<reqwest::Response> {
        if self.content_type == "json" {
            return client.json(&self.parse_data_to_json()).send().await;
        } else {
            return client.form(&self.parse_data_to_form()).send().await;
        }
    }

    pub async fn send_post(&self) -> Result<StatusCode, Box<dyn Error>> {
        let client = self.builder_client()?.post(self.url.clone());
        let resp = self.get_response(client).await?;
        let status = resp.status();
        if self.debug {
            println!("{:?}", resp.text().await?);
        }
        Ok(status)
    }
}
