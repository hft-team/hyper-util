use std::env;

use http_body_util::Empty;
use hyper::Request;
use hyper_util::client::legacy::{connect::HttpConnector, Client};

fn main() {
    let args: Vec<String> = env::args().collect();

    tokio::fstack_init(args.len(), args);

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();


    let url = String::from("http://httpbin.org/ip");
    let url = url.parse::<hyper::Uri>().expect("failed to parse URL");
    if url.scheme_str() != Some("http") {
        eprintln!("This example only works with 'http' URLs.");
    }

    let client = Client::builder(hyper_util::rt::TokioExecutor::new()).build(HttpConnector::new());

    let req = Request::builder()
        .uri(url)
        .body(Empty::<bytes::Bytes>::new())
        .expect("failed to build request");


    rt.block_on(async {
        let local = tokio::task::LocalSet::new();
        local.run_until(async move {
            let resp = client.request(req).await.expect("failed to fetch URL");
            eprintln!("{:?} {:?}", resp.version(), resp.status());
            eprintln!("{:#?}", resp.headers());
        }).await;
    });
}
