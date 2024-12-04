use std::io::{self, Write};
use tokio;
use anyhow::Result;

mod rigatoni; 
use rigatoni::{function, parameters, Message, OllamaClient, Property, Tool};

pub async fn add_two_numbers(a:i32, b:i32)->i32{
    a+b
}

pub async fn subtract_two_numbers(a:i32, b:i32)->i32{
    a-b
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut ai_model = OllamaClient::new();
    println!("Welcome to the AI CLI! Type your message and press Enter. Type 'exit' to quit.\n");
    let mut preamble: Vec<Message> = Vec::<Message>::new();
    let path = r#"C:\Users\tanis\Desktop\rigatoni\rigatoni\rigatoni\src\Modelfile"#;

    match ai_model
    .create_model("twitch", Some(path), None, None, false)
    .await
    {
        Ok(response) => {
            println!("Model 'twitch' created successfully: {:?}", response);
        }
        Err(err) => {
            eprintln!("Error creating model 'twitch': {}", err);
            return Err(err);
        }
    }

    ai_model.set_model("twitch");

    let tool = Tool{
        tool_type: "function".to_string(),
        function: function {
            name: "subtract_two_numbers".to_string(),
            description: "Subtract two numbers".to_string(),
            parameters: parameters{
                param_type: "object".to_string(),
                required: vec!["a".to_string(), "b".to_string()],
                properties:  std::collections::HashMap::from([("a".to_string(), Property{prop_type: "integer".to_string(), description: "First number".to_string()}), ("b".to_string(), Property{prop_type: "integer".to_string(), description: "Second number".to_string()})]),
            }

        }

    };
    
    ai_model.create_tool(tool);


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
