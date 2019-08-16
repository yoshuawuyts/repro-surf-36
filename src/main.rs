#![feature(async_await)]

#[runtime::main]
async fn main() {
    femme::start(log::LevelFilter::Info).unwrap();
    let string = surf::get("http://localhost:8080")
        .recv_string()
        .await
        .unwrap();
    dbg!(&string);
    println!("received {} bytes", string.len());
}
