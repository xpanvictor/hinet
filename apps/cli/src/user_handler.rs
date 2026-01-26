use tokio::io::{self, AsyncBufRead, AsyncRead, AsyncReadExt, AsyncWrite};

pub async fn handle_user_command() {
    // Placeholder for user command handling logic
    let mut input = String::new();
    loop {
        input.clear();
        print!("\nEnter command: ");
        // need to make it async read
        if tokio::io::stdin().read_to_string(&mut input).await.is_ok() {
            let command = input.trim();
            if command.eq_ignore_ascii_case("exit") || command.eq_ignore_ascii_case("quit") {
                println!("Exiting user command handler.");
                break;
            } else {
                println!("Received command: {}", command);
                // will now place runtime clap command handling here
            }
        } else {
            println!("Failed to read line from stdin.");
        }
    }
}
