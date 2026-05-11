use ReqUeST::client;
// use tokio;

#[tokio::main]
async fn main() {
    match client::get_page("https://doc.rust-lang.org/rust-by-example/hello.html").await {
        Ok(html) => println!("{}", html),
        Err(e) => eprintln!("Error: {}", e),
    }}
}

