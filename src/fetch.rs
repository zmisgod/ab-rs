use std::thread::{self};
use tokio::task::{self};
use clap::Parser;
use crate::http;

#[derive(Parser, Debug)]
#[command(name = "ab-rs")]
#[command(author, version, about="a benchmark by rust", long_about = None, )]
pub struct Args {
    // Seconds to max. wait for each response(millseconds)
    #[arg(short='t', default_value_t = 3000)]
    http_timeout: i32,

    // Number of requests to perform
    #[arg(short='n', default_value_t = 1)]
    request_num: i32,

    // http post data
    #[arg(short='d', default_value_t = String::from(""))]
    http_post_data: String,

    // http method
    #[arg(short='m', default_value_t = String::from("GET"))]
    http_method: String,

    // http headers
    // #[arg(short='r', default_value_t = Vec::new())]
    // http_header: Vec<String>,

    // is open debug mod? you can see logs
    #[arg(long="debug", default_value_t = false)]
    debug: bool,

    // url 
    url: String,
}

pub async fn run() {
    let args = Args::parse();
    let mut fetch = build_new_fetch(args.url);
    fetch.n = args.request_num;
    fetch.http.set_method(args.http_method.to_string());
    fetch.http.set_data(Vec::new());
    fetch.http.set_content_type(String::from(""));
    fetch.http.set_debug(args.debug);
    fetch.http.set_timeout(args.http_timeout);
    println!("{:?}", fetch);
    fetch.bench().await;
}

#[derive(Debug)]
pub struct Fetch {
    n: i32,
    http: http::Http,
}

impl Fetch {
    async fn bench(&mut self) {
        let mut i = 1;
        loop {
            if i > self.n {
                break;
            }
            let resp = self.http.clone().send_request().await;
            println!("{:?} -- fetch data-- {:?}", resp, i);
            i += 1;
        }
    }
}

pub fn build_new_fetch(_url: String) -> Fetch {
    Fetch {
        n: 0,
        http: http::build_new_http(_url),
    }
}