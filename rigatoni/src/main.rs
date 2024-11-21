use std::io::{self, Write};
use tokio;
use anyhow::Result;

// Include your OllamaClient and related structs here
mod rigatoni; // Assuming the client code is in `ollama_client.rs`
use rigatoni::OllamaClient;

#[tokio::main]
async fn main() -> Result<()> {
    let mut ai_model = OllamaClient::new();
    println!("Welcome to the AI CLI! Type your message and press Enter. Type 'exit' to quit.\n");

    loop {
        print!("You: ");
        io::stdout().flush()?; // Ensure the prompt is displayed immediately
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim(); // Remove trailing newline or whitespace

        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        match ai_model.completion(input).await {
            Ok(response) => {
                println!("AI: {}\n", response.message.unwrap().content);
            }
            Err(err) => {
                eprintln!("Error: {}\n", err);
            }
        }
    }

    Ok(())
}
