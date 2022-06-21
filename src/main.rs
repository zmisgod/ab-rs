use serde::Deserialize;
use std::env;
use std::error::Error;
use std::sync::mpsc;
use std::thread::{self};
use std::time::Duration;
use tokio::task;

#[derive(Debug)]
struct Fetch<'a> {
    n: i32,
    c: i32,
    url:&'a str,
}

#[derive(Debug, Deserialize)]
struct Response {
    msg: String,
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

    fn send_msg(&self) {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            tx.send(String::from("h1")).unwrap();
            tx.send(String::from("2222")).unwrap();
        });
        loop {
            let recived = rx.recv();
            match recived {
                Ok(v) => {
                    println!("Got: {:?}", v);
                }
                Err(e) => {
                    println!("err: {:?}", e);
                    break;
                }
            }
        }
    }

    fn do_multi_thread_fetch(&self) {
        let thread_one = std::thread::spawn(move || {
            println!("------111");
        });
        let thread_two = std::thread::spawn(move || {
            println!("------222");
        });
        thread_one.join().expect("one is panicked");
        thread_two.join().expect("one is panicked");
    }
}

enum ParseDataCType {
    NType,
    CType,
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
    a
}

fn build_new_fetch(_url: &str) -> Fetch {
    Fetch {
        n: 0,
        c: 0,
        url: _url,
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

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut fetch = build_new_fetch(&args[1]);
        let new_args = args.clone();
        for value in new_args {
            for parse in all_parse_data().into_iter() {
                if value.contains(parse.key) {
                    let _n = value.replace(parse.key, "");
                    let mut _tep_c = _n.parse::<i32>().unwrap();
                    if _tep_c <= 0 {
                        _tep_c = 1
                    }
                    match parse.c_type {
                        ParseDataCType::CType => fetch.c = _tep_c,
                        ParseDataCType::NType => fetch.n = _tep_c,
                    }
                }
            }
        }
        let mut n: i32 = 0;
        while n != fetch.get_c() {
            task::spawn(fetch_data(args[1].to_string(), fetch.get_n()));
            n += 1;
        }
        thread::sleep(Duration::from_secs(10));
    }
}
