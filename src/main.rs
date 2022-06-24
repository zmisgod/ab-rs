use serde::Deserialize;
use std::env;
use std::error::Error;
//use std::sync::mpsc;
use std::thread::{self};
use std::time::Duration;
use tokio::task;
use std::fmt;

#[derive(Debug)]
struct Fetch<'a> {
    n: i32,
    c: i32,
    url:&'a str,
    method: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Response {
    msg: String,
}

#[allow(dead_code)]
struct GetReq {
    url:String,
}

#[allow(dead_code)]
struct PostReq {
    url:String,
    data:String,
}

#[allow(dead_code)]
enum Method {
    Get(GetReq),
    Post(PostReq),
}

impl fmt::Debug for Method {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Method debug")
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Method")
    }
}

async fn send_request(url: &String) -> Result<Response, Box<dyn Error>> {
    println!("{:?}", url);
    let resp = reqwest::get(url).await?;
    let resp_str = resp.json::<Response>().await?;
    Ok(resp_str)
}

impl<'a> Fetch<'a> {
    fn get_c(&self) -> i32 {
        self.c
    }

    fn get_n(&self) -> i32 {
        self.n
    }

    fn set_method(&mut self, _method: String) {
        self.method = _method
    }

    fn do_ben(&self) {
        let mut n: i32 = 0;
        while n != self.get_c() {
            task::spawn(fetch_data(self.url.to_string(), self.get_n()));
            n += 1;
        }
        thread::sleep(Duration::from_secs(5));
    }
}

enum ParseDataCType {
    NType,
    CType,
    IType
}

struct ParseData<'a> {
    key: &'a str,
    c_type: ParseDataCType,
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
        key: "-i",
        c_type: ParseDataCType::IType,
    });
    a
}

fn build_new_fetch(_url: &str) -> Fetch  {
    Fetch {
        n: 0,
        c: 0,
        url: _url,
        method:String::from("get"),
    }
}

async fn fetch_data(url: String, n: i32) {
    let mut i = 0;
    loop {
        if i > n {
            break;
        }
        let resp = send_request(&url).await;
        println!("{:?}", resp);
        i += 1;
    }
}

fn parse_line_params(args: Vec<String>, fetch: &mut Fetch) {
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
                    ParseDataCType::CType => {
                        fetch.c = _tep_c
                    }
                    ParseDataCType::NType => {
                        fetch.n = _tep_c
                    }
                    ParseDataCType::IType => {
                        fetch.set_method(_clone.clone())
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut fetch = build_new_fetch(&args[1]);
        parse_line_params(args.clone(), &mut fetch);
        println!("{:?}", fetch);
        fetch.do_ben();
    }
}
