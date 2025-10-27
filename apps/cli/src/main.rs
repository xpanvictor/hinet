use root;

#[tokio::main]
async fn main() {
    println!("System initialize..");
    let rt = root::runtime::Runtime::run();
    match rt.await {
        Ok(_) => {println!("stable run")}
        Err(_) => {eprintln!("some error occurred")}
    }
}
