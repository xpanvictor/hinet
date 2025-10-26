use root;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let rt = root::runtime::Runtime::run();
    match rt.await {
        Ok(_) => {println!("stable run")}
        Err(_) => {eprintln!("some error occurred")}
    }
}
