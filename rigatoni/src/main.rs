use std::io::{self, Write};
use tokio;
use anyhow::Result;

mod rigatoni; 
use rigatoni::{Message, OllamaClient};

#[tokio::main]
async fn main() -> Result<()> {
    let mut ai_model = OllamaClient::new();
    println!("Welcome to the AI CLI! Type your message and press Enter. Type 'exit' to quit.\n");
    let mut preamble: Vec<Message> = Vec::<Message>::new();
    ai_model.set_model("llama3.1:8b");


    loop {
        print!("You: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        preamble.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });

        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        match ai_model.chat(preamble.clone()).await {
            Ok(response) => {
                let reply = response.message.unwrap().content;
                println!("AI: {}\n", reply);
                preamble.push(Message {
                    role: "assistant".to_string(),
                    content: reply.to_string(),
                });
            }
            Err(err) => {
                eprintln!("Error: {}\n", err);
            }
        }
    }

    Ok(())
}
