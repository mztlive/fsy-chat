use rig::{completion::ToolDefinition, tool::Tool};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
#[error("Math error")]
pub struct MathError;

pub struct Adder;

#[derive(Deserialize)]
pub struct AddArgs {
    x: i32,
    y: i32,
}

impl Tool for Adder {
    const NAME: &'static str = "add";

    type Error = MathError;

    type Args = AddArgs;

    type Output = i32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "add".to_string(),
            description: "Add x and y together".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "x": {
                        "type": "number",
                        "description": "The first number to add"
                    },
                    "y": {
                        "type": "number",
                        "description": "The second number to add"
                    }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("call adder");
        Ok(args.x + args.y)
    }
}
