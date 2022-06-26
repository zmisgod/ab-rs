use std::env;
mod fetch;
mod http;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut fetch = fetch::build_new_fetch(args[1].to_string());
        fetch::parse_line_params(args.clone(), &mut fetch);
        println!("{:?}", fetch);
        fetch.do_ben();
        // fetch.do_request().await;
    }
}
