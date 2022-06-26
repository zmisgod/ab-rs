use reqwest::StatusCode;
use std::error::Error;
use std::fmt;
use std::collections::HashMap;

pub struct Http {
    url: String,
    method: String,
    data: Vec<String>,
    content_type: String,
    show_body:bool,
}

impl Clone for Http {
    fn clone(&self) -> Http {
        Http {
            url: self.url.clone(),
            method: self.method.clone(),
            data: self.data.clone(),
            content_type: self.content_type.clone(),
            show_body:self.show_body,
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
            "url:{:?} method:{:?} data:{:?} content_type:{:?} show_body:{:?}",
            self.url, method, self.data, content_type, self.show_body
        )
    }
}

pub fn build_new_http(url: String) -> Http {
    Http {
        url: url,
        method: String::from(""),
        data: vec![],
        content_type: String::from(""),
        show_body:false
    }
}

impl Http {
    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_content_type(&mut self, content_type: String) {
        self.content_type = content_type;
    }

    pub fn set_show_body(&mut self, show:bool) {
        self.show_body = show
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
        }else{
            return self.send_get().await;
        }
    }

    async fn send_get(&self) -> Result<StatusCode, Box<dyn Error>> {
        let resp = reqwest::get(self.url.clone()).await?;
        let status = resp.status();
        if self.show_body {
            println!("{:?}", resp.text().await?);
        }
        Ok(status)
    }

    fn parse_data_to_form(&self) -> Vec<(String, String)> {
        let mut data_vec = vec![];
        for (_, val) in self.data.iter().enumerate() {
            let mut one:(String, String) = (String::from(""), String::from(""));
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
            let mut k =String::from("");
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

    pub async fn send_post(&self) -> Result<StatusCode, Box<dyn Error>>  {
        let client = reqwest::Client::new().post(self.url.clone());
        let resp;
        if self.content_type == "json" {
            resp = client.json(&self.parse_data_to_json()).send().await?;
        }else{
            resp = client.form(&self.parse_data_to_form()).send().await?;
        }
        let status = resp.status();
        if self.show_body {
            println!("{:?}", resp.text().await?);
        }
        Ok(status)
    }
}
