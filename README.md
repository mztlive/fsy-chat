# FSY AI 聊天

FSY AI 聊天是一个基于 Rust 开发的智能对话应用程序，支持文档检索增强生成(RAG)和智能对话功能。该应用程序可以在 CLI 或 Web 模式下运行，提供流式响应和文档知识库支持。

## 功能特点

- **双模式运行**：支持 CLI 和 Web 两种运行模式
- **流式响应**：实时流式输出 AI 回复内容
- **文档检索增强**：通过向量数据库实现高效的文档检索和问答
- **会话管理**：支持多会话管理和会话历史记录
- **文档管理**：支持分类别的文档加载和管理
- **向量存储**：使用 Qdrant 实现高效的向量存储和检索
- **可配置性**：通过 TOML 配置文件实现灵活配置

## 技术栈

- Rust 语言开发
- Tokio 异步运行时
- Axum Web 框架
- Qdrant 向量数据库
- OpenAI 兼容 API 接口
- 阿里云 DashScope 模型支持

## 安装

### 前置依赖

- Rust 1.65+
- Qdrant 向量数据库
- 阿里云 DashScope API 密钥或其他 OpenAI 兼容 API 密钥

### 从源代码构建

```bash
git clone https://github.com/yourusername/fsy-ai-chat.git
cd fsy-ai-chat
cargo build --release
```

## 配置

在项目根目录创建`config.toml`文件，内容示例：

```toml
# Qdrant向量数据库URL
qdrant_url = "http://localhost:6334"

# 代理配置
[agent]
# API密钥
api_key = "your-api-key"
# 代理前置指令
preamble = "你是一个专业的AI助手，..."
# 使用的聊天模型
chat_model = "qwen-max"

# 嵌入模型配置（可选）
[embedding]
api_key = "your-api-key"
model = "text-embedding-v1"
dimensions = 1536

# 文档配置
[document]
# 文档类别配置
[[document.categories]]
name = "知识库1"
directory = "./documents/knowledge1"
collection_name = "knowledge1"

[[document.categories]]
name = "知识库2"
directory = "./documents/knowledge2"
collection_name = "knowledge2"
```

## 使用方法

### CLI 模式

```bash
# 使用默认配置文件
./fsy-ai-chat

# 指定配置文件
./fsy-ai-chat -c custom_config.toml
```

### Web 模式

```bash
# 在3000端口启动Web服务
./fsy-ai-chat -m web -p 3000
```

## 文档格式

文档应以 JSON 格式存储，每个文件应包含一个或多个 JSON 对象数组，示例：

```json
[
  {
    "id": "doc1",
    "department": "技术部",
    "category": "常见问题",
    "question": "如何重置密码？",
    "question_variants": ["密码忘记了怎么办", "如何修改密码"],
    "answer": "您可以通过以下步骤重置密码：1. 访问登录页面 2. 点击'忘记密码' 3. 按照提示操作"
  }
]
```

## 开发

### 项目结构

- `src/main.rs` - 应用程序入口点
- `src/agent.rs` - AI 代理和模型交互
- `src/chat.rs` - 聊天会话管理
- `src/document_loader.rs` - 文档加载和管理
- `src/vector_store.rs` - 向量存储和检索
- `src/config.rs` - 配置结构定义
- `src/errors.rs` - 错误处理
- `src/web/` - Web 服务相关模块
- `src/tools/` - 工具函数模块

### 扩展功能

- 添加新工具：在`src/tools/`目录中添加新的工具实现
- 支持新文档类型：扩展`DocumentManager`以支持更多文档格式
- 添加新的 UI：扩展 Web 界面或开发新的前端

## 许可证

[MIT 许可证](LICENSE)
