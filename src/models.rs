use rig::Embed;
use serde::{Deserialize, Serialize};

/// 文档结构体
///
/// 表示可嵌入向量存储的文档，包含ID和消息内容
///
/// # 示例
/// ```
/// use fsy_ai_chat::models::Document;
///
/// fn example() {
///     let doc = Document {
///         id: "doc_1".to_string(),
///         message: "这是一个示例文档内容".to_string(),
///     };
///     
///     println!("文档ID: {}, 内容: {}", doc.id, doc.message);
/// }
/// ```
#[derive(Embed, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Document {
    /// 文档唯一标识符
    pub id: String,

    /// 文档内容，将被用于嵌入向量化
    #[embed]
    pub message: String,
}
