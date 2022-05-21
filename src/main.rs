use clap::Parser;
use dotenv::dotenv;

mod command;
mod request;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Show hello message
    #[clap(long)]
    hello: bool,

    /// Show env
    #[clap(short, long, default_value = "USER")]
    env: String,

    /// Send get request
    #[clap(long)]
    request: bool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = Args::parse();

    if args.hello {
        println!("Hello, world!");
    }

    let key = args.env;
    let val = utils::get_env(&key);
    println!("{} = {}", key, val);

    command::hello_world();

    if args.request {
        let url = "https://google.com";
        let res = request::send_get_request(url).await;
        assert!(res.is_ok());
    }
}
