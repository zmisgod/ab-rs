use crate::http;
use serde::Deserialize;
use std::fmt;
use std::thread::{self};
use std::time::Duration;
use tokio::task::{self};

#[derive(Debug)]
pub struct Fetch {
    n: i32,
    c: i32,
    http: http::Http,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Response {
    msg: String,
}

#[allow(dead_code)]
struct GetReq {
    url: String,
}

#[allow(dead_code)]
struct PostReq {
    url: String,
    data: Vec<String>,
}

#[allow(dead_code)]
enum Method {
    Get(GetReq),
    Post(PostReq),
}

impl fmt::Debug for Method {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Method Debug")
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Method Display")
    }
}

impl Fetch {
    pub fn get_c(&self) -> i32 {
        self.c
    }

    pub fn get_n(&self) -> i32 {
        self.n
    }

    pub fn do_ben(&self) {
        let mut n: i32 = 0;
        while n != self.get_c() {
            task::spawn(fetch_data(self.http.clone(), self.n, n));
            n += 1;
        }
        thread::sleep(Duration::from_secs(5));
    }

    pub async fn do_request(&self) {
        let resp = self.http.clone().send_request().await;
        println!("{:?}", resp);
    }
}

struct ParseData<'a> {
    key: &'a str,
    c_type: ParseDataCType,
}

fn str_to_vec_string(str: String) -> Vec<String> {
    let res = str.split(",").map(|s| s.to_string()).into_iter();
    let mut vec_re: Vec<String> = Vec::new();
    for val in res {
        vec_re.push(val);
    }
    vec_re
}

enum ParseDataCType {
    NType,  //请求数
    CType,  //并行数
    HMType, //http的method，默认 get，可选post
    HDType, //http的data，仅当post有效，get请求直接把参数放在url中
    HCType, //http的contentType，仅当post有效,默认空
    HDebug,
}

fn all_parse_data() -> Vec<ParseData<'static>> {
    let mut a = Vec::new();
    a.push(ParseData {
        key: "-n",
        c_type: ParseDataCType::NType,
    });
    a.push(ParseData {
        key: "-c",
        c_type: ParseDataCType::CType,
    });
    a.push(ParseData {
        key: "-hm",
        c_type: ParseDataCType::HMType,
    });
    a.push(ParseData {
        key: "-hd",
        c_type: ParseDataCType::HDType,
    });
    a.push(ParseData {
        key: "-hc",
        c_type: ParseDataCType::HCType,
    });
    a.push(ParseData {
        key: "-debug",
        c_type: ParseDataCType::HDebug,
    });
    a
}

pub fn build_new_fetch(_url: String) -> Fetch {
    Fetch {
        n: 0,
        c: 0,
        http: http::build_new_http(_url),
    }
}

async fn fetch_data(http: http::Http, n: i32, index :i32) {
    let mut i = 1;
    loop {
        if i > n {
            break;
        }
        let resp = http.clone().send_request().await;
        println!("{:?} -- fetch data {:?} -- {:?}", resp, index, i);
        i += 1;
    }
}

pub fn parse_line_params(args: Vec<String>, fetch: &mut Fetch) {
    for value in args {
        for parse in all_parse_data().into_iter() {
            if value.contains(parse.key) {
                let _n = value.replace(parse.key, "");
                let _clone = _n.clone();
                let mut _tep_c = _n.parse::<i32>().unwrap_or_default();
                if _tep_c <= 0 {
                    _tep_c = 1
                }
                let mut _tem_str = _n;
                match parse.c_type {
                    ParseDataCType::CType => fetch.c = _tep_c,
                    ParseDataCType::NType => fetch.n = _tep_c,
                    ParseDataCType::HMType => fetch.http.set_method(_clone.clone()),
                    ParseDataCType::HDType => {
                        fetch.http.set_data(str_to_vec_string(_clone.clone()));
                    }
                    ParseDataCType::HCType => {
                        fetch.http.set_content_type(_clone.clone());
                    }
                    ParseDataCType::HDebug => {
                        fetch.http.set_show_body(true)
                    }
                }
            }
        }
    }
}
