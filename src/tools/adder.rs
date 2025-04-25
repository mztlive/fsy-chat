use rig::{completion::ToolDefinition, tool::Tool};
use serde::Deserialize;
use serde_json::json;

/// 数学错误类型
///
/// 表示数学计算过程中可能出现的错误
#[derive(Debug, thiserror::Error)]
#[error("Math error")]
pub struct MathError;

/// 加法工具
///
/// 提供简单的加法计算功能
pub struct Adder;

/// 加法参数
///
/// 包含要相加的两个数字
#[derive(Deserialize)]
pub struct AddArgs {
    /// 第一个加数
    x: i32,
    /// 第二个加数
    y: i32,
}

impl Tool for Adder {
    const NAME: &'static str = "add";

    type Error = MathError;

    type Args = AddArgs;

    type Output = i32;

    /// 获取工具定义
    ///
    /// # 参数
    /// * `_prompt` - 提示信息，在此工具中未使用
    ///
    /// # 返回值
    /// 返回描述工具参数和功能的定义对象
    ///
    /// # 示例
    /// ```
    /// use rig::tool::Tool;
    /// use fsy_ai_chat::tools::adder::Adder;
    ///
    /// async fn example() {
    ///     let adder = Adder;
    ///     let definition = adder.definition("计算两个数的和".to_string()).await;
    ///     println!("工具名称: {}", definition.name);
    /// }
    /// ```
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

    /// 执行加法计算
    ///
    /// # 参数
    /// * `args` - 包含两个加数的参数对象
    ///
    /// # 返回值
    /// 返回两个数字的和，或在计算出错时返回错误
    ///
    /// # 示例
    /// ```
    /// use rig::tool::Tool;
    /// use fsy_ai_chat::tools::adder::{Adder, AddArgs};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let adder = Adder;
    ///     let args = AddArgs { x: 5, y: 3 };
    ///     let result = adder.call(args).await?;
    ///     assert_eq!(result, 8);
    ///     Ok(())
    /// }
    /// ```
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("call adder");
        Ok(args.x + args.y)
    }
}
